use std::process;
fn main() {
    if let Err(e) = biggrep::run(biggrep::BigGrepArgs::new()) {
        println!("Anwendungsfehler: {e}");
        process::exit(1);
    }
}
