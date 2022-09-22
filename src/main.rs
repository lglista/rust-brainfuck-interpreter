mod interpreter;
mod syntax_checker;
mod active_environment;
use std::env;

use crate::interpreter::interpreter::Interpreter;
use crate::syntax_checker::syntax_checker::SyntaxChecker;
use crate::active_environment::active_environment::ActiveEnvironment;
use std::fs;

fn main() {
    let argv: Vec<_> = env::args().collect();
    let save_input_in_file: bool = argv[1].eq("-s") || argv[1].eq("--save-commands");

    let mut ae = ActiveEnvironment::new(save_input_in_file);
    ae.start_environment();

    if save_input_in_file {
        fs::write("foo.bf", ae.input_to_save).expect("Unable to write file");
    }
}