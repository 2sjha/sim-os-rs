CC = gcc
CFLAGS = -std=c11

computer.exe: computer.o computer.h cpu.o load.o memory.o shell.o
	$(CC) $(CFLAGS) computer.o cpu.o load.o memory.o shell.o -o computer.exe

computer.o: computer.c
	$(CC) $(CFLAGS) -c computer.c -o computer.o

memory.o: memory.c
	$(CC) $(CFLAGS) -c memory.c -o memory.o

cpu.o: cpu.c
	$(CC) $(CFLAGS) -c cpu.c -o cpu.o

load.o: load.c
	$(CC) $(CFLAGS) -c load.c -o load.o

shell.o: shell.c
	$(CC) $(CFLAGS) -c shell.c -o shell.o

.PHONY: clean
clean:
	rm *.o
	rm computer.exe
