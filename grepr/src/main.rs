pub mod lib;

fn main() {
    if let Err(e) = grepr::get_args().and_then(grepr::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
