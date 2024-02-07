use rust_ml_toolkit::manager::manager::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = get_option();
    startapp(options)
}
