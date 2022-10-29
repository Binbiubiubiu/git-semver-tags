#![deny(missing_docs)]

//! # Documentation: Derive Reference
//!
//! > Get all git semver tags of your repository in reverse chronological order
//!

use lazy_static::lazy_static;
use regex::Regex;
use std::{io::Write, process::Command};

mod cli;
#[macro_use]
mod macros;
pub use cli::Args;

/// isleerna
fn is_lerna_tag<'a>(tag: &'a str, pkg: &Option<String>) -> bool {
    lazy_static! {
        static ref RE: Regex = format_regex!(r"^.+@[0-9]+\.[0-9]+\.[0-9]+(-.+)?$");
    }
    if let Some(pkg) = pkg {
        return format_regex!(r"^{}@", pkg).is_match(tag);
    } else {
        return RE.is_match(tag);
    }
}

/// semver_valid
fn semver_valid(version: &str) -> bool {
    semver::Version::parse(version).is_ok()
}

/// git_semver_tags
pub fn captures(args: &Args) -> Vec<String> {
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
    } = args;

    let tag_prefix_re = tag_prefix
        .as_ref()
        .map(|prefix| format_regex!(r"^{}(.*)", prefix));

    let mut binding = Command::new("git");
    binding.args(["log", "--decorate", "--no-color"]);
    if let Some(cwd) = cwd {
        binding.current_dir(cwd);
    }
    let output = binding.output().expect("failed to execute process");
    if !output.stderr.is_empty() {
        std::io::stderr().write_all(&output.stderr).unwrap();
        std::process::exit(1);
    }
    let output = String::from_utf8(output.stdout).expect("convert to String fail");
    return output
        .split("\n")
        .map(|decorations| {
            TAG_RE
                .captures_iter(decorations)
                .filter_map(|cap| {
                    let tag = &cap[1];

                    if *skip_unstable && UNSTABLE_TAG_RE.is_match(tag) {
                        return None;
                    }

                    if *lerna {
                        if is_lerna_tag(tag, &package) {
                            return Some(tag.to_string());
                        }
                    } else if let Some(re) = tag_prefix_re.as_ref() {
                        let captures = re.captures(tag);
                        if matches!(captures, Some(cap) if semver_valid(&cap[1])) {
                            return Some(tag.to_string());
                        }
                    } else if semver_valid(tag) {
                        return Some(tag.to_string());
                    }
                    None
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
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
}
