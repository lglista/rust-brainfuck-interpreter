pub mod interpreter{
    use std::io;
    use std::io::Write;

    const SIZE_OF_ARRAY: usize = 30000;

    pub struct Interpreter{
        pub array: [u8; SIZE_OF_ARRAY],
        pub index: usize,
        pub output: char,
        pub loops: Vec<(usize, usize)>
    }

    impl Interpreter{
        pub fn new() -> Interpreter {
            let vec1 = Vec::new();
            let i = Interpreter{
                array: [0; SIZE_OF_ARRAY],
                index: 0,
                output: '\0',
                loops: vec1};
            return i;
        }

        pub fn reset(&mut self) {
            for i in 0..SIZE_OF_ARRAY {
                self.array[i] = 0;
            }
            self.index = 0;
            self.output = '\0';
            self.loops = Vec::new();
        }

        pub fn interpret(&mut self, program: &str) {
            let mut index = 0usize;
            while index < program.len() {
                let command = program.chars().nth(index).unwrap();
                if command == '>' {
                    self.increment_pointer();
                }
                else if command == '<' {
                    self.decrement_pointer();
                }
                else if command == '+' {
                    self.increment_value();
                }
                else if command == '-' {
                    self.decrement_value();
                }
                else if command == '.' {
                    self.output_value();
                }
                else if command == ',' {
                    self.read_input(program.chars().nth(index + 1).unwrap());
                    index += 1;
                }
                else if command == '[' {
                    index = self.read_open_bracket(program, index);
                }
                else if command == ']' {
                    index = self.read_close_bracket(index);
                }
                index += 1;
            }
        }

        fn increment_pointer(&mut self) {
            self.index += 1;
            if self.index == SIZE_OF_ARRAY {
                self.index = 0;
            }
        }

        fn decrement_pointer(&mut self) {
            if self.index == 0 {
                self.index = SIZE_OF_ARRAY - 1;
            }
            else {
                self.index -= 1;
            }
        }

        fn increment_value(&mut self) {
            if self.array[self.index] == u8::MAX {
                self.array[self.index] = 0
            }
            else {
                self.array[self.index] += 1;
            }
        }

        fn decrement_value(&mut self) {
            if self.array[self.index] == 0 {
                self.array[self.index] = u8::MAX;
            }
            else {
                self.array[self.index] -= 1;
            }
        }

        fn output_value(&mut self) {
            self.output = self.array[self.index] as char;
            print!("{}", self.output);
            io::stdout().flush().unwrap();
        }

        fn read_input(&mut self, character: char) {
            self.array[self.index] = character as u8;
        }

        fn read_open_bracket (&mut self, program: &str, index: usize) -> usize {
            self.loops.push((index, 0));

            if self.array[self.index] == 0 {
                if self.loops[self.loops.len() - 1].1 != 0 {
                    return self.loops.pop().unwrap().1;
                }
                else {
                    let mut num_opens = 0;
                    let mut temp_index = index + 1;
                    while temp_index < program.len() {
                        if num_opens == 0 && program.chars().nth(temp_index).unwrap() == ']' {
                            return temp_index;
                        }
                        else if program.chars().nth(temp_index).unwrap() == '[' {
                            num_opens += 1;
                        }
                        else if program.chars().nth(temp_index).unwrap() == ']' {
                            num_opens -= 1;
                        }
                        temp_index += 1
                    }
                    return program.len();
                }
            }

            return self.loops[self.loops.len() - 1].0;
        }

        fn read_close_bracket (&mut self, index: usize) -> usize {
            let last_element_of_loops = self.loops.len() - 1;
            self.loops[last_element_of_loops].1 = index;

            if self.array[self.index] != 0 {
                return self.loops[last_element_of_loops].0;
            }
            else {
                self.loops.pop();
                return index;
            }
        }
    }


    #[test]
    fn test_instantiaion() {
        let interpreter = Interpreter::new();
        for i in 0..SIZE_OF_ARRAY{
            assert_eq!(interpreter.array[i], 0);
        }
        assert_eq!(interpreter.index, 0);
    }

    #[test]
    fn test_increment_pointer() {
        let mut interpreter = Interpreter::new();
        interpreter.interpret(">");
        assert_eq!(interpreter.index, 1);

        for _ in 0..SIZE_OF_ARRAY{
            interpreter.interpret(">");
        }
        assert_eq!(interpreter.index, 1);
    }

    #[test]
    fn test_decrement_pointer() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("<");
        assert_eq!(interpreter.index as usize, SIZE_OF_ARRAY - 1);

        interpreter.interpret("<");
        assert_eq!(interpreter.index as usize, SIZE_OF_ARRAY - 2);
    }

    #[test]
    fn test_increment_value() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("+");
        assert_eq!(interpreter.array[interpreter.index], 1);

        interpreter.interpret("+");
        assert_eq!(interpreter.array[interpreter.index], 2);
    }

    #[test]
    fn test_decrement_value() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("-");
        assert_eq!(interpreter.array[interpreter.index], u8::MAX);

        interpreter.interpret("-");
        assert_eq!(interpreter.array[interpreter.index], u8::MAX - 1);
    }

    #[test]
    fn test_output() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret(".");
        assert_eq!(interpreter.output, 0u8 as char);

        interpreter.interpret(">+.");
        assert_eq!(interpreter.output, 1u8 as char);
    }

    #[test]
    fn test_input() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret(",a");
        assert_eq!(interpreter.array[interpreter.index], 'a' as u8);

        interpreter.interpret(",+");
        assert_eq!(interpreter.array[interpreter.index], '+' as u8);
    }

    #[test]
    fn test_reset() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("+++[>+<-]");
        interpreter.interpret("+++>+++<[>+<->>[>+<-]<<]");

        interpreter.reset();

        for i in 0..SIZE_OF_ARRAY{
            assert_eq!(interpreter.array[i], 0);
        }
        assert_eq!(interpreter.index, 0);
        assert_eq!(interpreter.output, '\0');
        assert_eq!(interpreter.loops.len(), 0);
    }

    #[test]
    fn test_loop() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("+++[-]+");
        assert_eq!(interpreter.array[0], 1);
        interpreter.reset();

        interpreter.interpret("+++[>+<-]");
        assert_eq!(interpreter.array[1], 3);
        interpreter.reset();

        interpreter.interpret("+++>+++<[>+<-]");
        assert_eq!(interpreter.array[0], 0);
        assert_eq!(interpreter.array[1], 6);
        interpreter.reset();

        interpreter.interpret("+++>+++<[>+<->>[>+<-]<<]");
        assert_eq!(interpreter.array[1], 6);
    }

    #[test]
    fn test_hello_world() {
        let mut interpreter = Interpreter::new();

        interpreter.interpret("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.
        >+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");

        assert_eq!(interpreter.array[0], 0);
        assert_eq!(interpreter.array[1], 87);
        assert_eq!(interpreter.array[2], 100);
        assert_eq!(interpreter.array[3], 33);
        assert_eq!(interpreter.array[4], 10);
    }
}