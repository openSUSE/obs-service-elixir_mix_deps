use crate::errors::SourceServiceError;

use std::env;
use std::path::Path;
use std::process::Command;

pub fn fetch_mix_deps(subdir: &str) -> Result<(), SourceServiceError> {
    let origin = env::current_dir()?;
    let root = Path::new(subdir);

    env::set_current_dir(root)?;

    Command::new("mix").args(["deps.get"]).output()?;

    env::set_current_dir(origin)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn fetch_mix_deps_successful() {
        let result = fetch_mix_deps("test/test_project");
        fs::remove_dir_all("test/test_project/deps").unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn fetch_mix_deps_error() {
        let result = fetch_mix_deps("test/carbonara");
        assert!(result.is_err());
    }
}
