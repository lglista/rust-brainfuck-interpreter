/// Created by Lawrence Glista 09/20/2022
pub mod active_environment{
    use crate::Interpreter;
    use crate::SyntaxChecker;
    use std::io::{self, BufRead, Write};

    pub struct ActiveEnvironment {
        interpreter: Interpreter,
        syntax_checker: SyntaxChecker,
        save_input_in_file: bool,
        pub input_to_save: String
    }

    impl ActiveEnvironment {
        pub fn new(save_input: bool) -> ActiveEnvironment {
            let i = Interpreter::new();
            let s = SyntaxChecker::new();
            let inp = String::new();
            let ae = ActiveEnvironment{interpreter: i, syntax_checker: s,
                save_input_in_file: save_input, input_to_save: inp};
            return ae;
        }

        pub fn start_environment(&mut self) {
            self.read_and_process_input();
        }

        fn read_and_process_input(&mut self) {
            let line = self.read_input();
            self.process(&line);
        }

        fn read_input(&mut self) -> String {
            print!("bf> ");
            io::stdout().flush().unwrap();
            let stdin = io::stdin();
            let line: String = stdin.lock().lines().next().unwrap().unwrap();
            return line;
        }

        fn process(&mut self, line: &str) {
            if line.len() == 1 {
                let character: char = line.chars().nth(0).unwrap();
                match character {
                    'p' => self.print_status(),
                    'q' => return,
                    'r' => self.interpreter.reset(),
                    'h' => self.print_help(),
                     _  => self.interpret_line(line)
                }
            }
            else {
                self.interpret_line(line);
            }
            self.read_and_process_input();
        }

        fn interpret_line(&mut self, line: &str) {
            if self.syntax_checker.syntax_is_correct(line) {
                self.interpreter.interpret(line);
                if self.save_input_in_file {
                    self.input_to_save.push_str(&(line.to_string() + "\n"));
                }
                
            }
        }

        fn print_help(&self) {
            println!("Brainfuck Active Environment Help");
            println!("Enter a brainfuck program through stdin by one command at a time or in a bunch");
            println!("Options must appear on a line by themselves");
            println!("Options:");
            println!("\th: print this help");
            println!("\tp: print the status of the Brainfuck Active Environment");
            println!("\tr: reset the Brainfuck Active Environment");
            println!("\tq: quit");
        }

        fn print_status(&self) {
            // 11 values per 80 chars
            println!("Brainfuck Active Environment Status");
            println!("Current index: {}", self.interpreter.index);
            println!("Current value at index: {0} {1}",
            self.interpreter.array[self.interpreter.index], self.interpreter.array[self.interpreter.index] as char);

            let mut number_values: String = "".to_owned();
            let mut array_numbers: String = "".to_owned();
            for num in -5..6 {
                let correct_index = self.get_index(num);
                number_values.push_str(&self.correct_length_string(&self.interpreter.array[correct_index].to_string(), 7u8));
                array_numbers.push_str("[");
                array_numbers.push_str(&self.correct_length_string(&correct_index.to_string(), 5u8));
                array_numbers.push_str("]");
            }

            println!("{}", number_values);
            println!("{}", array_numbers);
            println!("                                      ^                                      ");
        }

        fn get_index(&self, num_to_add: i8) -> usize {
            let mut val: usize = self.interpreter.index;
            if num_to_add < 0 {
                for _ in 0..num_to_add.abs() {
                    if val == 0 {
                        val = 30000;
                    }
                    val -= 1;
                }
            }
            else {
                for _ in 0..num_to_add {
                    if val == 29999 {
                        val = 0;
                    }
                    else {
                        val += 1;
                    }
                }
            }

            return val;
        }

        fn correct_length_string(&self, val: &str, length: u8) -> String {
            let mut tmp: String = val.to_string();
            while tmp.len() != length as usize{
                tmp = tmp + &" ".to_owned();
                if tmp.len() != length as usize{
                    tmp = " ".to_owned() + &tmp;
                }
            }
            return tmp;
        }
    }
}