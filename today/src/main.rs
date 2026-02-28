

fn main() {
    if let Err(e) = today::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
