fn main() {
    if let Err(e) = translate_parser::get_args().and_then(translate_parser::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}