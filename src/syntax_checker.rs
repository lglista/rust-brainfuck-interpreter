pub mod syntax_checker{
    pub struct SyntaxChecker{}

    impl SyntaxChecker{
        pub fn new() -> SyntaxChecker {
            return SyntaxChecker{};
        }

        pub fn syntax_is_correct(&self, program: &str) -> bool {
            return self.program_is_not_empty(program) && self.input_has_entry(program) &&
                   self.brackets_are_matching(program);
        }

        fn program_is_not_empty(&self, program: &str) -> bool {
            return program.len() != 0;
        }

        fn input_has_entry(&self, program: &str) -> bool {
            if program.chars().nth(program.len() - 1).unwrap() == ',' {
                if program.len() == 1 ||
                   program.chars().nth(program.len() - 2).unwrap() != ','{
                    println!("ERROR: Input command ',' at the end of the program does not read a value.");
                    println!("Make sure that each Input command ',' has a value after it.");
                    return false;
                }
            }
            return true;
        }

        fn brackets_are_matching(&self, program: &str) -> bool {
            let mut stack = Vec::new();
            for index in 0..program.len() {
                if index != 0 && program.chars().nth(index - 1).unwrap() == ',' {
                    continue;
                }
                if program.chars().nth(index).unwrap() == '[' {
                    stack.push(index);
                }
                if program.chars().nth(index).unwrap() == ']' {
                    if stack.len() == 0 {
                        println!("ERROR: Unmatched closed bracket ']' at program index {}", index);
                        return false;
                    }
                    stack.pop();
                }
            }

            if stack.len() != 0 {
                for index in 0..stack.len() {
                    println!("ERROR: Unmatched open bracket '[' at program index {}", stack[index]);
                }
                return false;
            }
            return true;
        }
    }

    #[test]
    fn test_input_check(){
        let sc = SyntaxChecker::new();

        assert_eq!(sc.syntax_is_correct(","), false);
        assert_eq!(sc.syntax_is_correct(",."), true);
        assert_eq!(sc.syntax_is_correct(",,"), true);
    }

    #[test]
    fn test_brackets_matching() {
        let sc = SyntaxChecker::new();

        assert_eq!(sc.syntax_is_correct("[]"), true);
        assert_eq!(sc.syntax_is_correct("["), false);
        assert_eq!(sc.syntax_is_correct("]"), false);

        assert_eq!(sc.syntax_is_correct("[,]"), false);
        assert_eq!(sc.syntax_is_correct(",[]"), false);
        assert_eq!(sc.syntax_is_correct("[,]]"), true);
        assert_eq!(sc.syntax_is_correct(",[[]"), true);
    }

    #[test]
    fn test_empty_program() {
        let sc = SyntaxChecker::new();

        assert_eq!(sc.syntax_is_correct(""), false);
    }
}