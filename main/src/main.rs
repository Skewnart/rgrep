use std::process;

fn main() {
    if let Err(err) = lib::run(){
        eprintln!("{err}");
        process::exit(1);
    }
}
