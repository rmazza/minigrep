use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Search {
    pub query: String,
    pub directory: String,
    pub case_insensitive: bool,
}

impl Search {
    fn search(        
        dir: &Path,
        query: &str,
        case_insensitive: bool,
        cb: &dyn Fn(&str)) -> Result<(), Box<dyn Error>>{
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
                    Self::search(&path_buf.as_path(), query, case_insensitive, cb)?;
                }
            }
            Ok(())
    }

    fn print(path: &str) {
        println!("{}", path);
    }
}

pub trait Command {
    fn new(args: &[String]) -> Self;
    fn run(&self);
}

impl Command for Search {
    fn new(args: &[String]) -> Search
    {
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
    
         {
            query,
            directory,
            case_insensitive,
        }
    }
        
    fn run(&self) {
        Search::search(Path::new(&self.directory), &self.query, self.case_insensitive, &Search::print);
    }
}