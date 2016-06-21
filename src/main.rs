extern crate git2;

use git2::Repository;

fn main() {
    let repo = match Repository::open('.') {
        Ok(repo) => Some(repo),
        Err(e) => None,
    }.unwrap();

    let head = match repo.head() {
        Ok(reference) => Some(reference),
        Err(e) => None,
    }.unwrap();

    let branch = match head.symbolic_target();

    if branch.is_some() {
        println!("{}", branch.unwrap());
    }
    else {
        println!("{}", head.name().unwrap());
    }
}
