use crate::errors::SourceServiceError;

use std::process::Command;

pub fn fetch_mix_deps(subdir: &str) -> Result<(), SourceServiceError> {
    Command::new("mix")
        .args(["deps.get"])
        .current_dir(subdir)
        .output()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn fetch_mix_deps_successful() {
        let result = fetch_mix_deps("tests/test_project");
        fs::remove_dir_all("tests/test_project/deps").unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn fetch_mix_deps_error() {
        let result = fetch_mix_deps("tests/carbonara");
        assert!(result.is_err());
    }
}
