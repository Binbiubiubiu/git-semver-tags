use std::{env::current_dir, path::Path};

use assert_fs::prelude::*;

macro_rules! exec {
    ($cmd:expr$(,$arg:expr)*) => {{
        let mut cmd = assert_cmd::Command::new($cmd);
        cmd$(.arg($arg))*;
        cmd.ok()
    }}
}

macro_rules! git_semver_tags {
    ()=>{{
        assert_cmd::Command::cargo_bin("git-semver-tags").unwrap().ok()
    }};
    ($head:expr$(,$arg:expr)*) => {{
        let mut bindings = assert_cmd::Command::cargo_bin("git-semver-tags").unwrap();
        bindings.arg($head);
        $(bindings.arg($arg);)*
        bindings.ok()
    }};
}

macro_rules! git_dummy_commit {
    ($head:expr$(,$arg:expr)*) => {{
        let mut cmd = assert_cmd::Command::new("git");
        cmd.arg("commit");
        cmd.args(&["-m",$head]);
        $(cmd.args(&["-m",$arg]);)*
        cmd.args(&["--allow-empty","--no-gpg-sign"]);
        cmd.ok()
    }}
}

fn write_file<P>(temp_dir: &assert_fs::TempDir, file_name: P, content: &str) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let file = temp_dir.child(file_name);
    file.touch()?;
    std::fs::write(file.path(), content)?;
    Ok(())
}

fn test_cli_case1() -> anyhow::Result<()> {
    let temp = assert_fs::TempDir::new()?;
    let pre_path = current_dir()?;
    std::env::set_current_dir(temp.path())?;
    exec!("git", "init")?;

    assert!(
        git_semver_tags!().is_err(),
        "should error if no commits found"
    );

    write_file(&temp, "test1", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"First-commit\"")?;
    exec!("git", "tag", "foo")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        "".to_owned(),
        "should get no semver tags"
    );

    write_file(&temp, "test2", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"Second commit\"")?;
    exec!("git", "tag", "v2.0.0")?;

    write_file(&temp, "test3", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"Third commit\"")?;
    exec!("git", "tag", "va.b.c")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v2.0.0", ""].join("\n"),
        "should get the semver tag"
    );

    exec!("git", "tag", "v3.0.0")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v3.0.0", "v2.0.0", ""].join("\n"),
        "should get both semver tags"
    );

    exec!("git", "tag", "v4.0.0")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v4.0.0", "v3.0.0", "v2.0.0", ""].join("\n"),
        "should get all semver tags if two tags on the same commit"
    );

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v4.0.0", "v3.0.0", "v2.0.0", ""].join("\n"),
        "should still work if I run it again"
    );

    write_file(&temp, "test4", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"Fourth commit\"")?;
    exec!("git", "tag", "v1.0.0")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v1.0.0", "v4.0.0", "v3.0.0", "v2.0.0", ""].join("\n"),
        "should still work if I run it again"
    );

    std::env::set_current_dir(pre_path)?;
    temp.close()?;
    Ok(())
}

fn test_cli_case2() -> anyhow::Result<()> {
    let temp = assert_fs::TempDir::new()?;
    let pre_path = current_dir()?;
    std::env::set_current_dir(temp.path())?;
    exec!("git", "init")?;

    git_dummy_commit!("empty commit")?;
    exec!("git", "tag", "v1.1.0")?;
    exec!("git", "tag", "blarg-project@1.0.0")?;
    git_dummy_commit!("empty commit2")?;
    git_dummy_commit!("empty commit2")?;

    let output = git_semver_tags!()?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v1.1.0", ""].join("\n"),
        "should work with empty commit"
    );

    write_file(&temp, "test5", "2")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"sixth commit\"")?;
    exec!("git", "tag", "foo-project@4.0.0")?;
    write_file(&temp, "test5", "3")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"seventh commit\"")?;
    exec!("git", "tag", "foo-project@5.0.0")?;

    let output = git_semver_tags!("--lerna")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        [
            "foo-project@5.0.0",
            "foo-project@4.0.0",
            "blarg-project@1.0.0",
            ""
        ]
        .join("\n"),
        "should work with lerna style tags"
    );

    write_file(&temp, "test5", "4")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"fifth commit\"")?;
    exec!("git", "tag", "foobar-project@0.0.10")?;
    write_file(&temp, "test5", "5")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"sixth commit\"")?;
    exec!("git", "tag", "foobar-project@0.10.0")?;
    write_file(&temp, "test5", "6")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"seventh commit\"")?;
    exec!("git", "tag", "foobar-project@10.0.0")?;

    let output = git_semver_tags!("--lerna")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        [
            "foobar-project@10.0.0",
            "foobar-project@0.10.0",
            "foobar-project@0.0.10",
            "foo-project@5.0.0",
            "foo-project@4.0.0",
            "blarg-project@1.0.0",
            ""
        ]
        .join("\n"),
        "should work with lerna style tags with multiple digits"
    );

    write_file(&temp, "test5", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"seventh commit\"")?;
    exec!("git", "tag", "bar-project@5.0.0")?;

    let output = git_semver_tags!("--lerna", "--package", "bar-project")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["bar-project@5.0.0", ""].join("\n"),
        "should allow lerna style tags to be filtered by package"
    );

    let binding = git_semver_tags!("--package", "bar-project").err().unwrap();
    let output = binding.as_output().unwrap().to_owned();
    let suffix = if cfg!(windows) { ".exe" } else { "" };
    assert_eq!(
        String::from_utf8(output.stderr)?,
        format!("error: The following required arguments were not provided:\n  --lerna\n\nUsage: git-semver-tags{suffix} --lerna --package <package>\n\nFor more information try \'--help\'\n"),
        "should not allow package filter without lernaTags=true"
    );
    write_file(&temp, "test6", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"eigth commit\"")?;
    exec!("git", "tag", "ms/6.0.0")?;
    write_file(&temp, "test6", "1")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"tenth commit\"")?;
    exec!("git", "tag", "ms/7.0.0")?;
    write_file(&temp, "test6", "2")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"eleventh commit\"")?;
    exec!("git", "tag", "notms/7.0.0")?;

    let output = git_semver_tags!("--tag-prefix", "ms/")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["ms/7.0.0", "ms/6.0.0", ""].join("\n"),
        "should work with tag prefix option"
    );

    write_file(&temp, "test7", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"twelfth commit\"")?;
    exec!("git", "tag", "skip/8.0.0")?;
    write_file(&temp, "test8", "1")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"thirteenth commit\"")?;
    exec!("git", "tag", "skip/9.0.0-alpha.1")?;
    write_file(&temp, "test9", "2")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"fourteenth commit\"")?;
    exec!("git", "tag", "skip/9.0.0-rc.1")?;
    write_file(&temp, "test10", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"fifteenth commit\"")?;
    exec!("git", "tag", "skip/9.0.0")?;

    let output = git_semver_tags!("--tag-prefix", "skip/", "--skip-unstable")?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["skip/9.0.0", "skip/8.0.0", ""].join("\n"),
        "should skip unstable tags"
    );

    std::env::set_current_dir(pre_path)?;
    temp.close()?;
    Ok(())
}

fn test_cli_case3() -> anyhow::Result<()> {
    let temp = assert_fs::TempDir::new()?;
    let pre_path = current_dir()?;
    std::env::set_current_dir(temp.path())?;

    let binding = temp.join("footer");
    let footer_dir = binding.as_path();
    std::fs::create_dir(footer_dir)?;
    std::env::set_current_dir(footer_dir)?;
    exec!("git", "init")?;

    write_file(&temp, "footer/test2", "")?;
    exec!("git", "add", "--all")?;
    exec!("git", "commit", "-m", "\"First commit\"")?;
    exec!("git", "tag", "v1.1.0")?;

    std::env::set_current_dir(temp.path())?;
    let output = git_semver_tags!("--cwd", footer_dir.to_str().unwrap())?;
    assert_eq!(
        String::from_utf8(output.stdout)?,
        ["v1.1.0", ""].join("\n"),
        "git semver tags on different cwd"
    );

    std::env::set_current_dir(pre_path)?;
    temp.close()?;
    Ok(())
}

#[test]
fn test_cli() -> anyhow::Result<()> {
    test_cli_case1()?;
    test_cli_case2()?;
    test_cli_case3()?;
    Ok(())
}
