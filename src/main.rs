mod intepreter;
mod parse;

extern crate clap;
use clap::*;
use std::fs;

fn main() {
    let matches = App::new("Migraine CLI")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true)
                .value_name("INPUT FILE"),
        )
        .get_matches();

    if matches.is_present("input") {
        let input_file_path = matches.value_of("input").unwrap();
        let content = fs::read_to_string(input_file_path).unwrap();
        let res = parse::parse(content);
        if let Err(err) = res {
            eprintln!("ERROR: {:?}", err);
        } else {
            println!("Running:");
            let mut runtime = intepreter::Runtime::new(res.unwrap());
            let res = runtime.run_func_with_name("main".to_string());
            
            // if runtime returns an error, print
            if let Err(res) = res {
                eprintln!("{}", res);
            }
        }
    }
}
