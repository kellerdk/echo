use clap::Parser;

fn main() {
    if let Err(e) = Ok(echo::Config::parse()).and_then(echo::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
