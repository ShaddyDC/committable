use std::io::{stdin, Read};

use committable_lib::{check_all_rules, Commit};
use miette::Result;

fn main() -> Result<()> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    check_all_rules(&Commit::new(&buffer))?;

    println!("No errors in commit message {buffer:?}!");

    Ok(())
}
