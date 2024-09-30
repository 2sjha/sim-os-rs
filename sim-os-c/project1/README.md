# Simulated OS - Project 1

In Project 1, we have a simulated computer with [CPU](./cpu.c), [Memory](./memory.c), [Shell](./shell.c), and [Loader](./load.c).
The configuration for this simulated computer is provided in `config.sys` which just specifies memory size. Once the simulated computer starts, it shows a simulated shell prompt. Allowed Shell commands, and simulated CPU instructions are given below.

`make` creates the simulated computer executable `computer.exe`.

`execute` shell script provides the [input program](./comp.in) to be run on this simulated computer, Output is printed in [comp.out](./comp.out).

## Note for UT Dallas students

This was Project 1 for UTD's CS5348 course.
Please don't use this code as is, because it may be flagged for plagiarism. UTD CS department takes plagiarism very seriously.
Please refer to [UTD's Academic Dishonesty](https://conduct.utdallas.edu/dishonesty) page for more info.

## Instructions for Simulated CPU
----
| OP Code   | Operand   | System actions |
|-----------|-----------|----------------|
| 1 (load)  | constant  | Load the constant to AC |
| 2 (load)  | m-addr    | Load Mem[m-addr] into AC |
| 3 (add)   | m-addr    | Load Mem[m-addr] into MBR, add MBR to AC |
| 4 (mul)   | m-addr    | Same as above, except that add becomes multiply |
| 5 (store) | m-addr    | Store AC to Mem[m-addr] |
| 6 (ifgo)  | m-addr    | If AC > 0 then go to the address given in Mem[m-addr] Otherwise, continue to the next instruction |
| 7 (print) | Null      | Print the value in AC |
| 8 (sleep) | Time      | Sleep for the given “time” in microseconds, which is the operand |
| 9 (shell) | Code      | Execute the shell command according to code (elaborated later) |
| 0 (exit)  | Null      | End of the current program, null is 0 and is unused |
----

## Shell command codes for `9` CPU instruction

- 2 : Print Registers
- 3 : Print Memory

## File descriptions

1. [computer.c](./computer.c)
- This file has the main function which reads mem_size from config.sys,
  then initializes the CPU and Memory in boot_system function.
- The program expects the user-program-filename and Base address as input.
  If computer.exe is executed without any args, it will prompt for the input.
  If computer.exe is executed with input as CLI arguments, input is read from there.
- Calls the Loader to load the program in memory
- Creates PCB
- Starts CPU Operation
- Calls load_finish to cleanup

2. [computer.h](./computer.h)
- Contains prototypes of functions which are used accross the project.
- Also contains the struct definitions

3. config.sys
- contains an integer which is the total size of memory

4. [cpu.c](./cpu.c)
- Contains the RegisterFile struct object which contains the state of all 7 registers
- cpu_mem_address function which accepts the mem-address as input, calculates the actual memory address and calls the mem_read function to read from memory, and returns MBR value
- cpu_fetch _instruction reads memory address from PC and PC+1 to IR0 and IR1, also updates PC to PC + 2.
- cpu_execute_instruction reads IR0 and IR1 and does the appropriate data movement
- cpu_operation is an infinite loop of fetching and executing instructions until program end is read at IR0 = 0

5. [execute](./execute)
- executes the make command and calls 2 execution of computer.exe as mentioned in the spec

6. [load.c](./load.c)
- load_prog accepts the user-prgram filename and base-address, loads the program and the data into memory word by word.
- load_finish closes the File.

7. [makefile](./makefile)
- create object files for each .c file
- link .o files into computer.exe

8. [memory.c](./memory.c)
- contains the Memory struct object which emulates the memory
- mem_init takes the size of memory, and initializes the memory array
- mem_read and mem_write are used to read from and write to memory using MAR and MBR 

9. [shell.c](./shell.c)
- shell_print_registers prints the state of all registers
- shell_print_memory prints the state of the memory
