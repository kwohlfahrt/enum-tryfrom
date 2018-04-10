extern crate compiletest_rs as compiletest;

// Doesn't seem to work when not in root/tests

#[test]
fn compile_fail() {
    use std::path::PathBuf;
    let mut config = compiletest::Config::default();
    config.mode = compiletest::common::Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile-fail");
    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config)
}
