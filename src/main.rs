use std::fs::{File};
use std::path::Path;
use std::io::{BufReader, BufRead};

use docopt::Docopt;
use path_absolutize::*;

const USAGE: &'static str = "
Usage:  locs <dir>
";

#[derive(Debug)]
struct Count {
    total_lines: u32,
    non_empty_lines: u32,
    empty_lines: u32,

}

fn count_lines(path: &Path) -> Count {
    let mut total_lines = 0;
    let mut non_empty_lines = 0;

    let input = File::open(&path);
    let content = match input {
        Ok(value) => Some(value),
        Err(error) => {
            println!("{:?}", error);
            println!("Cannot open {:?}", &path);
            None
        }
    };

    if content.is_some() {
        let content2 = content.unwrap();
        let buffered = BufReader::new(content2);
        for line in buffered.lines() {
            if let Ok(ip) = line {
                if ip != ""{
                    non_empty_lines += 1
                }
                total_lines += 1;
            }
        }
    }

    Count {
        total_lines, 
        non_empty_lines, 
        empty_lines: total_lines - non_empty_lines
    }
}

fn main() {
    let args = Docopt::new(USAGE)
                        .and_then(|d| d.parse())
                        .unwrap_or_else(|e| e.exit());
    let dir: &str = args.get_str("<dir>");
    let absolute_dir: &Path = &Path::new(dir).absolutize().unwrap(); 
    
    println!("{}", absolute_dir.display());
    

    let count = count_lines(absolute_dir);
    println!("{:#?}", count);
    println!("Total Lines: {}", count.total_lines);
    println!("Non-Empty Lines: {}", count.non_empty_lines);
    println!("Empty Lines: {}", count.empty_lines);
}
