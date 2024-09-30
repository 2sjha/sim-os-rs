# Simulated OS

This was the final OS project built on top of [Project 1](./project1/) and [Project 2](./project2/). The simulated OS has a simulated [CPU](./cpu.c), [Memory](./memory.c), [Shell](./shell.c), and a Round-Robin [scheduler](./scheduler.c). This also has a [simulated printer](./printer.c) which operates over the network with a printer manager that handles print instructions for each simulated process by spooling each process's print instructions.

When the simulated computer starts, it prompts the user with a simulated shell. Allowed shell commands are given below. When a user inputs a program (simulated process) to be run, the user must also provide the base address for the program's PCB block. We assume that the user will provide the correct base addresses to avoid memory overlap between programs loaded in the memory. Multiple programs running are scheduled using a Round-Robin scheduler. Whenever a simulated process issues a print instruction, it is communicated to the simulated printer via the TCP socket. The printer spools each process's print instructions, and at the process exit, all spooled print instructions are printed together on the [simulated paper](./printer.out).

Furthermore, the printer being on the network now allows to have multiple computers(identified by c_id) to connect to this printer and the printer manager handles each process's spools for each computer. The printer manager uses bounded buffer synchronization for incoming socket connections, and it also implements bounded buffers for each computer's print instructions via message queues. The simulated paper printer now utilizes the Readers-Writer synchronization while reading from the message queues and printing to the paper. Please refer [Semaphore Synchronization Design](./DESIGN.md) for more details.

## Note for UT Dallas students

This was Project 3 for UTD's CS5348 course.
Please don't use this code as is, because it may be flagged for plagiarism. UTD CS department takes plagiarism very seriously.
Please refer to [UTD's Academic Dishonesty](https://conduct.utdallas.edu/dishonesty) page for more info.

## How to run

- `make` or `make debug` to build release and debug executable `computer.exe` respectively.

- `./computer.exe 0`
this instance of computer.exe will act as the printer server. Printer server has its own version of shell which only supports 0 & 6 shell command.

- `./computer.exe 2`
this instance of computer.exe will act as a client computer with CID = 2.

- `./computer.exe`
With no CID, will prompt to input CID.

## Simulated shell Commands

- 0 : Shut Down computer
- 1 : Run program
- 2 : Print Registers
- 3 : Print Memory
- 4 : Print Scheduler ReadyQ
- 5 : Print PCB List
- 6 : Print Printer Spools

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
