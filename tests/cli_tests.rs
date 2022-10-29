#[test]
fn example_tests() {
    let t = trycmd::TestCases::new();
    t.case("tests/cmd/*.trycmd").case("tests/cmd/*.toml");
    t.extend_vars([("[EXAMPLE]", "example")]).unwrap();
    // t.register_bin("git-semver-tags", trycmd::schema::Bin::Name("git-semver-tags".to_owned()));
}