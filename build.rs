use std::{env, path::Path};

// build.rs
use dotenvy::from_path;

fn main() {
    let secrets_path = Path::new("secrets.env");
    let example_path = Path::new(".env.example");

    if secrets_path.exists() {
        // Load secrets.env if it exists
        from_path(secrets_path).unwrap();
    } else if example_path.exists() {
        // Otherwise, load .env.example as fallback
        from_path(example_path).unwrap();
        println!("Warning!!! Example Private Key loaded !!!");
    } else {
        // Neither file exists - optionally return error or ignore
        panic!("No secrets.env or .env.example found.");
    }

    let key = env::var("PRIVATE_KEY").unwrap();

    println!("cargo:rustc-env=PRIVATE_KEY={}", key);
}
