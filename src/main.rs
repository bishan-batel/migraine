mod parse;
mod intepreter;

extern crate clap;
use clap::*;

fn main() {
    let res = parse::parse();
    if let Err(err) = res {
        eprintln!("ERROR: {:?}", err);
    } else {
        for ele in res.unwrap() {
            println!("{:?}\n", ele);
        }
    }
}
