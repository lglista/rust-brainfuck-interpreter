# rust-brainfuck-interpreter
Brainfuck interpreter that accepts input line by line through a prompt.

Run with `cargo run` from the top level directory, or run the executable created by `cargo build` in target/debug/. To allow for command line arguments with the former, you must run `cargo run -- [args]`.

This interpreter keeps state through each command. See below for a list of commands.

# Command Line Arguments
| Command | Explanation |
| ------- | ----------- |
| --save-commands, -s | Passing this command saves the brainfuck commands executed throughout the lifetime of the program as well as any tape resets. A filename is expected after this argument to save the commands to. This file will be created/overwritten in the directory that this program is run in. If no filename is supplied, the default filename is `foo.bf`.

# Brainfuck commands
| Command | Explanation |
| ------- | ----------- |
|    +    | Increment the byte at the pointer |
|    -    | Decrement the byte at the pointer |
|    >    | Increment the data pointer |
|    <    | Decrement the data pointer |
|    .    | Output the byte at the data pointer |
|    ,    | Accept a byte of input at the data pointer. Example: `,h` will put the value 104 at the data pointer. This differs in regular brainfuck syntax in that input is inlined rather than taken from stdin. |
|    \[    | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching \] command.
|    \]    | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching \[ command. |

# Interpreter specific commands
These must be the only thing entered on a line in order for them to run
| Command | Explanation |
| ------- | ----------- |
|    p    | Print the status of the tape |
|    r    | Reset the status of the tape |
|    h    | Print the Interpreter options |
|    q    | Quit the program |

