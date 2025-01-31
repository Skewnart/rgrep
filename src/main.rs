use std::process;

fn main() {
    if let Err(err) = rgrep::run(){
        eprintln!("{err}");
        process::exit(1);
    }
}
