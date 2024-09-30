CC = gcc
CFLAGS = --std=c11 -lpthread

BUILD_DIR := ./build

all: computer.exe

debug: CFLAGS += -DDEBUG -g
debug: computer.exe

computer.exe: computer.o cpu.o memory.o print.o printer.o scheduler.o shell.o utils.o
	$(CC) $(CFLAGS) $(BUILD_DIR)/computer.o $(BUILD_DIR)/cpu.o $(BUILD_DIR)/memory.o $(BUILD_DIR)/print.o $(BUILD_DIR)/printer.o $(BUILD_DIR)/scheduler.o $(BUILD_DIR)/shell.o $(BUILD_DIR)/utils.o -o computer.exe

computer.o: computer.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c computer.c -o $(BUILD_DIR)/computer.o

cpu.o: cpu.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c cpu.c -o $(BUILD_DIR)/cpu.o

memory.o: memory.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c memory.c -o $(BUILD_DIR)/memory.o

print.o: print.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c print.c -o $(BUILD_DIR)/print.o

printer.o: printer.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c printer.c -o $(BUILD_DIR)/printer.o

scheduler.o: scheduler.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c scheduler.c -o $(BUILD_DIR)/scheduler.o

shell.o: shell.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c shell.c -o $(BUILD_DIR)/shell.o

utils.o: utils.c computer.h
	mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -c utils.c -o $(BUILD_DIR)/utils.o

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
	rm computer.exe
