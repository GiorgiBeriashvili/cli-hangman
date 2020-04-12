extern crate tera;
#[macro_use]
extern crate lazy_static;

use std::{error::Error, fs, path::Path, process, str};
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("resources/templates/**/*") {
            Ok(tera) => tera,
            Err(error) => {
                println!("Parsing error(s): {}", error);
                ::std::process::exit(1);
            }
        };

        tera
    };
}

fn main() {
    render_templates();
}

fn render_templates() {
    let mut context = Context::new();

    let package_version = env!("CARGO_PKG_VERSION");
    let rustc_version = get_command_output("rustc", &["--version"]);

    let rustc_version = str::from_utf8(&rustc_version.stdout[6..]).unwrap().trim();

    context.insert("package_version", &package_version);
    context.insert("rustc_version", &rustc_version);

    match TEMPLATES.render("README.tera", &context) {
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

fn get_command_output(command: &str, args: &[&str]) -> process::Output {
    match process::Command::new(command).args(args).output() {
        Ok(output) => output,
        Err(error) => {
            println!("{}", &error);

            std::process::exit(1);
        }
    }
}
