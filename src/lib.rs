#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub, missing_docs, rust_2018_idioms)]
#![doc = include_str!("../README.md")]

use lazy_static::lazy_static;
use regex::Regex;
#[cfg(feature = "self_upgrade")]
use self_update::cargo_crate_version;
use std::{io::Write, process::Command};

mod cli;
#[macro_use]
mod macros;
pub use cli::{Args, Commands};

fn is_lerna_tag(tag: &str, pkg: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = format_regex!(r"^.+@[0-9]+\.[0-9]+\.[0-9]+(-.+)?$");
    }
    if let Some(pkg) = pkg {
        return format_regex!(r"^{}@", pkg).is_match(tag);
    }

    RE.is_match(tag)
}

fn semver_valid(mut version: &str) -> bool {
    if version.starts_with('v') {
        version = &version[1..]
    }
    semver::Version::parse(version).is_ok()
}

/// upgrade self version
#[cfg(feature = "self_upgrade")]
#[cfg_attr(docsrs, doc(cfg(feature = "self_upgrade")))]
pub fn self_upgrade(is_test: bool) -> Result<(), Box<dyn std::error::Error>> {
    let binding = clap::crate_authors!().split('<').collect::<Vec<_>>();
    let authors = binding.first().expect("get author name");
    let status = self_update::backends::github::Update::configure()
        .repo_owner(authors)
        .repo_name(clap::crate_name!())
        .bin_name(clap::crate_name!())
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(is_test)
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

/// List the git tags in the project
///
/// # Examples
///
/// ```no_run
/// use git_semver_tags::{captures, Args};
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let args = Args::default();
///     for tag in captures(&args).iter() {
///         println!("{}", tag);
///     }
///     Ok(())
/// }
/// ```
pub fn captures(args: Args) -> Vec<String> {
    lazy_static! {
        static ref TAG_RE: Regex = format_regex!(r"tag:\s*(.+?)[,)]");
        static ref UNSTABLE_TAG_RE: Regex = format_regex!(r".+-\w+\.\d+$");
    }

    let Args {
        tag_prefix,
        lerna,
        package,
        cwd,
        skip_unstable,
        ..
    } = args;

    let tag_prefix_re = tag_prefix.map(|prefix| format_regex!(r"^{}(.*)", prefix));

    let mut binding = Command::new("git");
    binding.args(["log", "--decorate", "--no-color"]);
    if let Some(cwd) = cwd {
        binding.current_dir(cwd);
    }
    let output = binding.output().expect("success to execute process");
    if !output.stderr.is_empty() {
        std::io::stderr()
            .write_all(&output.stderr)
            .expect("expose error msg");
        std::process::exit(1);
    }
    let output = String::from_utf8(output.stdout).expect("the stdout convert to String fail");
    return output
        .split('\n')
        .flat_map(|decorations| TAG_RE.captures_iter(decorations))
        .filter_map(|cap| match &cap[1] {
            tag if skip_unstable && UNSTABLE_TAG_RE.is_match(tag) => None,
            tag if lerna => match is_lerna_tag(tag, &package) {
                true => Some(tag.to_owned()),
                false => None,
            },
            tag if tag_prefix_re.is_some() => {
                let re = tag_prefix_re.as_ref().unwrap();
                match re.captures(tag) {
                    Some(cap) if semver_valid(&cap[1]) => Some(tag.to_owned()),
                    _ => None,
                }
            }
            tag if semver_valid(tag) => Some(tag.to_owned()),
            _ => None,
        })
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_valid() {
        assert!(semver_valid("2.0.0"));
        assert!(!semver_valid("2.0"))
    }

    #[test]
    fn test_is_lerna_tag() {
        assert!(is_lerna_tag("pkg@1.0.0", &None));
        assert!(!is_lerna_tag("1.0.0", &None));
        assert!(is_lerna_tag("pkg@1.0.0", &Some("pkg".to_string())));
        assert!(!is_lerna_tag("pkg1@1.0.0", &Some("pkg".to_string())));
    }

    // #[test]
    // fn test_self_upgrade() {
    //     assert!(self_upgrade(true).is_ok());
    // }
}
