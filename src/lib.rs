use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub directory: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() == 1 {
            return Err("not enough arguments");
        }

        let mut count: usize = 0;
        let mut case_insensitive = false;
        let mut query: String = String::new();
        let mut directory: String = String::new();

        loop {
            count += 1;
            if count >= args.len() {
                break;
            }

            let val: &str = args[count].as_str();

            if val.contains('-') {
                match val {
                    "-d" | "--directory" => {
                        count += 1;
                        directory = args[count].clone();
                    }
                    "-q" | "--query" => {
                        count += 1;
                        query = args[count].clone();
                    }
                    "-i" | "--case-insensitive" => {
                        case_insensitive = true;
                    }
                    _ => println!("ab"),
                }
            }
        }

        Ok(Config {
            query,
            directory,
            case_insensitive,
        })
    }

    pub fn search(
        dir: &Path,
        query: &str,
        case_insensitive: bool,
        cb: &dyn Fn(&str),
    ) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path_buf = entry.path();
            let path = path_buf.as_path().to_str().unwrap_or_else(|| "");

            if !query.is_empty() {
                if case_insensitive && path.to_lowercase().contains(query) {
                    cb(path);
                } else if path.contains(query) {
                    cb(path);
                }
            }

            if path_buf.is_dir() {
                Config::search(&path_buf.as_path(), query, case_insensitive, cb)?;
            }
        }
        Ok(())
    }
}
