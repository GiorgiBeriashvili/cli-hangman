use lazy_static::lazy_static;
use std::{error::Error, fs, path::Path, process, str};
use tera::{Context, Tera};

lazy_static! {
    static ref TEMPLATES: Tera = {
        match Tera::new("resources/templates/**/*") {
            Ok(tera) => tera,
            Err(error) => {
                println!("Parsing error(s): {}", error);
                ::std::process::exit(1);
            }
        }
    };
}

fn main() {
    render_templates();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

fn render_templates() {
    let mut context = Context::new();

    let package_version = env!("CARGO_PKG_VERSION");
    let rustc_version = process::Command::new("rustc")
        .args(&["--version"])
        .output()
        .expect("Failed to retrieve the current version of the Rust compiler.");

    let rustc_version = str::from_utf8(&rustc_version.stdout[6..])
        .expect("Failed to slice the vector.")
        .trim();

    context.insert("package_version", &package_version);
    context.insert("rustc_version", &rustc_version);

    match TEMPLATES.render("README.md", &context) {
        Ok(template) => {
            let path = Path::new("README.md");

            fs::write(&path, &template).unwrap();
        }
        Err(error) => {
            println!("Error: {}", error);

            let mut cause = error.source();

            while let Some(error) = cause {
                println!("Reason: {}", error);

                cause = error.source();
            }
        }
    };
}
