use clap::Parser;
use std::error::Error;
use std::io::{stdin, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug, Default, Clone)]
#[command(
    author = "D. Keller",
    version = "0.1.0",
    about = "rust echo",
    long_about = ""
)]

pub struct Config {
    #[arg(default_value_t = String::from(""), help = "input")]
    input: String,

    #[arg(short, long, default_value_t = false, help = "newline")]
    newline: bool,
}

pub fn run(args: Config) -> MyResult<()> {
    let h = Box::new(BufReader::new(stdin()));
    let lines = |x: Box<dyn BufRead>| BufRead::lines(x).map(|i| i.expect(""));
    match "" == args.input {
        true => match args.newline {
            false => lines(h).for_each(|x| println!("{x:}")),
            true => lines(h).for_each(|x| print!("{x:}")),
        },

        false => match args.newline {
            true => print!("{:}", args.input),
            false => println!("{:}", args.input),
        },
    }
    Ok(())
}
