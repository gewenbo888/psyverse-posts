//! Integration test that ensures the binary can be executed and
//! fails gracefully when the required environment variable is missing.

use assert_cmd::Command;
use std::env;

#[test]
fn missing_token_exits_with_error() {
    // Ensure the variable is not set for this test.
    env::remove_var("GITHUB_TOKEN");

    let mut cmd = Command::cargo_bin("psyverse_bounty_hunter").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("GITHUB_TOKEN environment variable not set"));
}
