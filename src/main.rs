extern crate git2;

use std::process::Command;
use git2::Repository;
use std::str;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => Some(repo),
        Err(e) => None,
    }.unwrap();

    let head = match repo.head() {
        Ok(reference) => Some(reference),
        Err(e) => None,
    }.unwrap();

    let branch = head.symbolic_target();

    if branch.is_some() {
        println!("symbolic: {}", branch.unwrap());
    }
    else {
        println!("direct: {}", head.name().unwrap());
    }

    let output = Command::new("git")
                         .arg("stash list")
                         .output()
                         .unwrap();
    println!("output: {}", str::from_utf8(&output.stdout).unwrap());

}
