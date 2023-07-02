use std::process::Command;
use std::{fs, io, path::Path};

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
    println!("Starting page indexing on port 3000...");

    let next_public_dir = "public/_pagefind";
    let dl_directory = "_temp_spider_downloads";

    // crawl the application and download contents
    let mut command = Command::new("spider");

    command
        .args(["--domain", "http://localhost:3000", "download"])
        .output()
        .expect("failed to crawl local website. Make sure the server is alive using `next dev` or `next start`.");

    // index static markup downloaded
    let mut command = Command::new("pagefind");

    command
        .args(["--source", &dl_directory, "--bundle-dir", next_public_dir])
        .output()
        .expect("failed to crawl local website. Make sure the server is alive using `next dev` or `next start`.");

    let tmp_public_dir = format!("{dl_directory}/{}", next_public_dir);

    // copy pagefind files to public directory
    match std::fs::create_dir_all(&next_public_dir) {
        Ok(_) => match copy_dir_all(&tmp_public_dir, next_public_dir) {
            _ => (),
        },
        _ => (),
    }

    // delete duplicate content in _pagefind directory for public
    match fs::remove_dir_all(format!("{dl_directory}/public")) {
        _ => (),
    }
}
