mod lib;

fn main() {
    if let Err(e) = lib::get_args().and_then(lib::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
