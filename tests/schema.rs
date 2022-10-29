#[test]
fn dump_schema() {
    let bin_path = snapbox::cmd::cargo_bin!("git-semver-tags");
    snapbox::cmd::Command::new(bin_path)
        .assert()
        .success()
        .stdout_eq_path("schema.json");
}