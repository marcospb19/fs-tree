mod cli;
#[allow(unused_variables, dead_code, unused_mut, unused_imports)]
mod dotao;
// #[export_macro]
mod macros;

fn main() {
    dotao::run();
}
