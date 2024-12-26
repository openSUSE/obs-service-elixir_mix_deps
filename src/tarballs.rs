use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::Write;

use crate::errors::SourceServiceError;

pub fn recompress(
    archive_name: &str,
    deps_directory: &str,
    compression: &str,
) -> Result<(), SourceServiceError> {
    let tar_gz = File::create(archive_name)?;

    let encoder: Box<dyn Write> = match compression {
        "zst" => Box::new(zstd::stream::write::Encoder::new(tar_gz, 0)?),
        _ => Box::new(GzEncoder::new(tar_gz, Compression::default())),
    };

    let mut tar = tar::Builder::new(encoder);

    tar.append_dir_all("deps", deps_directory)?;
    tar.finish()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn recompress_successful() {
        let result = recompress("vendor.tar.gz", "tests/test_compression", "gz");
        fs::remove_file("vendor.tar.gz").unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn recompress_error() {
        let result = recompress("vendor2.tar.gz", "tests/carbonara", "gz");
        fs::remove_file("vendor2.tar.gz").unwrap();
        assert!(result.is_err());
    }
}
