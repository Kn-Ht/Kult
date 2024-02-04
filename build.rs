#![allow(dead_code)]
#![allow(unused_imports)]

use std::io::{self, Error};

fn cmd(exec: &str, args: &[&str]) -> Result<(), String> {
    std::process::Command::new(exec)
        .args(args)
        .status()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(debug_assertions)]
fn main() {}

#[cfg(not(debug_assertions))]
fn main() -> Result<(), String> {
    cmd("trunk", &["build", "--release"])?;

    let git_path = std::path::Path::new("Kn-Ht.github.io");
    if !git_path.is_dir() {
        println!("cargo:warning=Kn-Ht.github.io not found, cloning...");
        cmd("git", &["clone", "--depth=1", "https://github.com/kn-ht/kn-ht.github.io"])?;
    }

    let dest_path = git_path.join("kult");

    let copy_options = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        ..Default::default()
    };

    fs_extra::copy_items(&["dist"], dest_path, &copy_options).map_err(|e| e.to_string())?;

    std::env::set_current_dir(git_path).map_err(|e| e.to_string())?;
    cmd("git", &["add", "."])?;
    cmd("git", &["commit", "-m", "\"(automated) update build\""])?;
    cmd("git", &["push"])?;

    std::env::set_var("KULT_BUILT", "1");
    Ok(())
}
