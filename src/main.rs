extern crate serde;

use serde::Deserialize;
use std::{env, error, fs, path::Path};
use toml;

const CONFIG: &str = "config.toml";

fn main() -> Result<(), Box<dyn error::Error>> {
    // TODO:
    // - Add error handling
    // - Pull out all to separate functions, structures etc
    // - Move all stuff to lib.rs
    // - Implement tests
    // - Unbind args from hardcoded args[1] position [V]
    // - Add a target as command line option and create a new field
    //   in the Config struct

    let config = Config::new(env::args())?;

    for path in config.dotfiles {
        let source_path = env::var("HOME").unwrap() + "/" + &path;

        let mut target_path = path.chars(); // create an iterator from str slice
        target_path.next(); // skip first element

        if Path::new(&source_path).exists() {
            print!("Copying {}...", &path);

            fs::copy(
                &source_path,
                // create subslice from original str slice with
                env::var("HOME").unwrap() + "/" + &config.target + "/" + &target_path.as_str(),
            )?;
        } else {
            eprintln!("warning: {} path doesn't exists.", path);
        }

        println!("Done");
    }

    Ok(())
}

// bring the Deserialize trait
// which transforms string into a struct
#[derive(Deserialize, Debug)]
struct Config {
    dotfiles: Vec<String>,
    target: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, Box<dyn error::Error>> {
        args.next();
        let mut dotfiles = vec![];
        let mut target = String::new();

        for arg in args {
            if arg == CONFIG {
                // read configuration file into a string
                let config_str = fs::read_to_string(&arg)?;

                // get .toml structure from string
                let config: Config = toml::from_str(&config_str)?;

                dotfiles = config.dotfiles; // shadows previous empty `dotfiles` var
                target = config.target;
            } else {
                return Err(Box::from("config.toml was not provided"));
            }
        }

        Ok(Config { dotfiles, target })
    }
}
