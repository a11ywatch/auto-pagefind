use clap::Parser;
use std::process::Command;
use std::{fs, io, path::Path};

/// Auto Pagefind converter for web servers that need static support. Perfect for tools like next.js to get the search and play easily.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The download directory for storing the static.html files
    #[arg(short, long)]
    download_dir: Option<String>,
    /// The website url
    #[arg(short, long)]
    url: Option<String>,
}

/// copy all files to another directory
fn copy_dir_all(src: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&destination)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), destination.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let url = match args.url {
        Some(u) => u,
        _ => String::from("http://localhost:3000"),
    };

    let download_dir = match args.download_dir {
        Some(u) => u,
        _ => String::from("_temp_spider_downloads"),
    };

    println!("Starting next-page at {url}...");

    let next_public_dir = "public/_pagefind";

    // crawl the application and download contents
    let mut command = Command::new("spider");

    command
        .args(["--domain", &url, "download"])
        .output()
        .expect("failed to crawl local website. Make sure the server is alive using `next dev` or `next start`.");

    // index static markup downloaded
    let mut command = Command::new("pagefind");

    command
        .args(["--source", &download_dir, "--bundle-dir", next_public_dir])
        .output()
        .expect("failed to crawl local website. Make sure the server is alive using `next dev` or `next start`.");

    let tmp_public_dir = format!("{download_dir}/{}", next_public_dir);

    // if the public folder exist create the files
    if Path::new(&next_public_dir).exists() {
        // copy pagefind files to public directory
        match std::fs::create_dir_all(&next_public_dir) {
            Ok(_) => match copy_dir_all(&tmp_public_dir, next_public_dir) {
                _ => (),
            },
            _ => (),
        }

        // delete duplicate content in _pagefind directory for public
        match fs::remove_dir_all(format!("{download_dir}/public")) {
            _ => (),
        }
    }
}
