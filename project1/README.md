# Simulated OS - Project 1

In Project 1, we have a simulated computer with [CPU](./cpu.rs), [Memory](./memory.rs), [Shell](./shell.rs), and [Loader](./load.rs).
The configuration for this simulated computer is provided in `config.sys` which just specifies memory size. Once the simulated computer starts, it shows a simulated shell prompt. Allowed Shell commands, and simulated CPU instructions are given below.

## How to run

`rustc main.rs -o computer.exe` creates the simulated computer executable `computer.exe`.

## Simulated shell Commands

- 0 : Shut Down computer
- 1 : Run program
- 2 : Print Registers
- 3 : Print Memory

## Instructions for Simulated CPU

- OP Code   : Operand   : System actions
- 1 (load)  : constant  : Load the constant to AC
- 2 (load)  : m-addr    : Load Mem[m-addr] into AC
- 3 (add)   : m-addr    : Load Mem[m-addr] into MBR, add MBR to AC
- 4 (mul)   : m-addr    : Same as above, except that add becomes multiply
- 5 (store) : m-addr    : Store AC to Mem[m-addr]
- 6 (ifgo)  : m-addr    : If AC > 0 then go to the address given in Mem[m-addr] Otherwise, continue to the next instruction
- 7 (print) : Null      : Print the value in AC
- 8 (sleep) : Time      : Sleep for the given “time” in microseconds, which is the operand
- 9 (shell) : Code      : Execute the shell command according to shell command code
- 0 (exit)  : Null      : End of the current program, null is 0 and is unused

## Shell command codes for `9` CPU instruction

- 2 : Print Registers
- 3 : Print Memory
