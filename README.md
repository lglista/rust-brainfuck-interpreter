# rust-brainfuck-interpreter
Brainfuck interpreter that accepts input line by line through a prompt.

Run with `cargo run` from the top level directory

This interpreter keeps state through each command. See below for a list of commands.

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

