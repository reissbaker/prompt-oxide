extern crate git2;

use git2::Repository;

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
    let repo = Repository::open(".").ok();
    repo.and_then(|repo| {
        repo.head().ok().and_then(|head| {
            match head.symbolic_target() {
                Some(branchname) => Some(String::from(branch_from_refname(branchname))),
                None => Some(String::from(branch_from_refname(head.name().unwrap()))),
            }
        })
    })
}

fn branch_from_refname(refname: &str) -> &str {
    refname.split("/").last().unwrap()
}

fn stash_depth() -> Option<u64> {
    let f = match File::open(".git/refs/stash") {
        Ok(file) => file,

        /*
         * If opening this file fails, check if we're in a git repo at all. If
         * we are, the stash depth is zero; if we're not, there is no stash
         * (because there is no git repo).
         */
        Err(_) => match Path::new(".git").exists() {
            true => return Some(0),
            false => return None,
        }
    };

    let reader = BufReader::new(f);
    let mut stash_size = 0;
    for _ in reader.lines() {
        stash_size += 1;
    }

    Some(stash_size)
}
