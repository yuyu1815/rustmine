fn assert_optional_runner_env(key: &str, actual: &str) {
    if let Ok(expected) = env::var(key) {
        assert_eq!(
            actual,
            expected.as_str(),
            "runner scope env {key} did not match"
        );
    }
}

fn assert_runner_scope(manifest_path: &str, manifest: &TestManifest) {
    assert_optional_runner_env("ORACLE_EXPECTED_MANIFEST", manifest_path);
    assert_optional_runner_env("ORACLE_EXPECTED_CASE_ID", &manifest.case_id);
    assert_optional_runner_env(
        "ORACLE_EXPECTED_RUST_TEST_TARGET",
        &manifest.rust_test_target,
    );
    assert_optional_runner_env("ORACLE_EXPECTED_RUST_TEST_NAME", &manifest.rust_test_name);
}
