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
