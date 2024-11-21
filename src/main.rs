mod errors;
mod mix;
mod tarballs;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "vendor.tar.gz")]
    archivename: String,

    #[arg(short, long)]
    outdir: Option<String>,

    #[arg(short, long)]
    subdir: String,

    #[arg(short, long, default_value = "gz")]
    compression: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let deps_directory = format!("{}/{}", &args.subdir, "deps");

    mix::fetch_mix_deps(&args.subdir)?;
    tarballs::recompress(&args.archivename, &deps_directory, &args.compression)?;

    Ok(())
}
