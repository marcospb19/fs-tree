#![feature(bool_to_option)]
mod app;
mod cli;
mod commands;
mod diff;
mod macros;
mod util;

fn main() {
    app::run_app();
}
