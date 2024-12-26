use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prints_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("obs-service-elixir_mix_deps")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}
