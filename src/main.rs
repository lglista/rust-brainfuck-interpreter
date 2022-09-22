mod interpreter;
mod syntax_checker;
mod active_environment;

use crate::interpreter::interpreter::Interpreter;
use crate::syntax_checker::syntax_checker::SyntaxChecker;
use crate::active_environment::active_environment::ActiveEnvironment;

fn main() {
    let mut ae = ActiveEnvironment::new();
    ae.start_environment();
}