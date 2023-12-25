use headr::{get_args, run};

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
