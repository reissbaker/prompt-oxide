use std::str;
use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if env::args().count() <= 1 {
        println!("must pass arguments");
        process::exit(3);
    }

    if !Path::new(".git").exists() {
        process::exit(2);
    }

    let argstring = env::args().nth(1).unwrap();
    let arg = argstring.as_str();
    let output = match arg {
        "branch" => branch(),
        "stash-depth" => stash_depth().map(|depth| format!("{}", depth)),
        _ => {
            println!("unknown argument {}", arg);
            process::exit(3);
        }
    };

    match output {
        Some(string) => println!("{}", string),
        None => process::exit(2),
    }
}

fn branch() -> Option<String> {
    let mut f = match File::open(".git/HEAD") {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Err(_) => return None,
        _ => (),
    }

    Some(branch_from_refname(&s))
}

fn branch_from_refname(refname: &String) -> String {
    let trimmed = refname.trim();
    let last = trimmed.split("/").last().unwrap();

    if last.len() != trimmed.len() {
        return String::from(last);
    }

    let mut commit = refname[..7].to_string();
    commit.push_str("...");
    commit
}

fn stash_depth() -> Option<u64> {
    let f = match File::open(".git/refs/stash") {
        Ok(file) => file,

        /*
         * If opening this file fails, check if we're in a git repo at all. If
         * we are, the stash depth is zero; if we're not, there is no stash
         * (because there is no git repo).
         */
        Err(_) => return Some(0),
    };

    let reader = BufReader::new(f);
    let mut stash_size = 0;
    for _ in reader.lines() {
        stash_size += 1;
    }

    Some(stash_size)
}
