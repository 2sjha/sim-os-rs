# Simulated OS - Project 2

Similar to Project 1, Project 2 has a simulated computer with [CPU](./cpu.c), [Memory](./memory.c), and [Shell](./shell.c). Furthermore, this adds a Round-Robin [scheduler](./scheduler.c) and to manage print commands, we have a simulated [Paper Printer](./printer.c). The paper printer acts as an offloaded component since it runs as a child process.

When the simulated computer starts, it prompts the user with a simulated shell. Allowed shell commands are given below. When the user inputs a program (simulated process) to be run, the user must also provide the base address for the program's PCB block. We assume that the user will provide the correct base addresses to avoid memory overlap between programs loaded in the memory. Multiple programs running are scheduled using a Round-Robin scheduler. Whenever a simulated process issues a print instruction, it is communicated to the simulated printer via the pipe. The printer spools each process's print instructions, and at the process exit all spooled print instructions are printed together on the [simulated paper](./printer.out).

## Note for UT Dallas students

This was Project 2 for UTD's CS5348 course.
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

## Simulated shell Commands

- 0 : Shut Down computer
- 1 : Run program
- 2 : Print Registers
- 3 : Print Memory
- 4 : Print Scheduler ReadyQ
- 5 : Print PCB List
- 6 : Print Printer Spools

## File descriptions

1. computer.c

- reads config.sys, boots system, initializes components
- Runs Scheduler and Shell Threads(only for the parent process)

2. computer.h

- has struct defintions and functions used accross files.

3. cpu.c

- executes instructions from memory
- has its reg_state, always executes this register state

4. memory.c

- provides functions to access memory

5. print.c

- Initalizes Pipe between Printer and itself
- has functions to send commands to printer

6. printer.c

- Receives all commands via pipe
- Maintains spool files for all processes.
- Prints spooled output to simulated paper on process-exit or shutdown

7. scheduler.c

- Maintails PCBs for all active processes
- Manages CPU execution via a CPU scheduling algorithm with Round Robin ready queue

8. shell.c

- Provides shell interface to interact with simulated computer
- Accepts process_submit, dump and terminate commands
- loads programs into memory

9. utils.c

- has utility functions to manipulate readyQ
