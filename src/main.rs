use std::str;
use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ERR_NOTGIT: i32 = 2;
const ERR_BROKEN: i32 = 3;

fn main() {
    if env::args().count() <= 1 {
        println!("must pass arguments");
        process::exit(ERR_BROKEN);
    }

    if !Path::new(".git").exists() {
        process::exit(ERR_NOTGIT);
    }

    let argstring = env::args().nth(1).unwrap();
    let arg = argstring.as_str();
    let output = match arg {
        "branch" => branch().map(|b| *b),
        "stash-depth" => Ok(format!("{}", stash_depth())),
        "combo" => {
            match branch() {
                Ok(branchbox) => {
                    let mut string = String::from("(");
                    string.push_str(&*branchbox);
                    string.push_str(")");
                    let depth = stash_depth();
                    if depth > 0 {
                        string.push_str(&format!(" [{}]", depth));
                    }
                    Ok(string)
                },
                Err(e) => Err(e),
            }
        },
        _ => {
            println!("unknown argument {}", arg);
            process::exit(ERR_BROKEN);
        }
    };

    match output {
        Ok(string) => println!("{}", string),
        Err(_) => process::exit(ERR_BROKEN),
    }
}

fn branch() -> Result<Box<String>, std::io::Error> {
    let mut f = try!(File::open(".git/HEAD"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(branch_from_refname(&s))
}

fn branch_from_refname(refname: &String) -> Box<String> {
    let trimmed = refname.trim();
    let pieces = trimmed.split("/").collect::<Vec<&str>>();

    if pieces.len() > 2 {
        let mut out = String::new();

        for i in 2..(pieces.len() - 1) {
            out.push_str(pieces[i]);
            out.push('/');
        }

        out.push_str(pieces[pieces.len() - 1]);

        return Box::new(out);
    }

    if pieces.len() > 1 {
        return Box::new(String::from(trimmed));
    }

    let mut commit = refname[..7].to_string();
    commit.push_str("...");
    Box::new(commit)
}

fn stash_depth() -> u64 {
    let f = match File::open(".git/refs/stash") {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let reader = BufReader::new(f);
    let mut stash_size = 0;
    for _ in reader.lines() {
        stash_size += 1;
    }

    stash_size
}
