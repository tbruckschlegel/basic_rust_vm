# A simple VM in Rust

# Overview

This is a hobby project that is a testbed for VM coding related stuff.
It implements a basic virtual machine (VM) that executes a set of instructions. It supports operations like arithmetic, comparisons, memory management, conditional jumps, function calls, and printing values. The VM runs a sequence of instructions, modifying registers and memory as specified by the program.

## Features

*   **Arithmetic Operations**: Add, subtract, multiply, divide, modulo, and comparison operations (equality, inequality, greater than, etc.).
*   **Memory Management**: Allocation and freeing of memory blocks. Storing and loading values from specific memory addresses.
*   **Control Flow**: Conditional and unconditional jumps based on register values or specific offsets.
*   **Registers**: The VM uses 8 registers (each a 32-bit integer) for computation.
*   **Function Calls**: Support for calling functions with a return mechanism using a stack.
*   **Printing**: Output values stored in registers.

## Table of Contents

1.  [Structs and Enums](#Structs-and-Enums)
2.  [VM Methods](#VM-Methods)
3.  [Main Program Execution](#Main-Program-Execution)
4.  [Instruction Set](#Instruction-Set)

## Structs and Enums

### `MemoryRegion`

This struct represents a memory block managed by the VM.


```
struct MemoryRegion {
    size: usize,   // Size of the allocated memory block
    data: Vec<u8>, // Raw byte data
}
```

### `Instruction`

The `Instruction` enum represents the different operations supported by the VM.

```
enum Instruction {
    SetReg(usize, i32),              // Set value in register
    Add(usize, usize, usize),        // Add two registers
    Sub(usize, usize, usize),        // Subtract two registers
    Mul(usize, usize, usize),        // Multiply two registers
    Div(usize, usize, usize),        // Divide two registers
    Mod(usize, usize, usize),        // Modulo two registers
    Eq(usize, usize, usize),         // Check equality between registers
    Neq(usize, usize, usize),        // Check inequality between registers
    Gt(usize, usize, usize),         // Greater than comparison
    Lt(usize, usize, usize),         // Less than comparison
    Gte(usize, usize, usize),        // Greater than or equal to comparison
    Lte(usize, usize, usize),        // Less than or equal to comparison
    Jump(usize),                     // Jump to a specific instruction
    JumpIfZero(usize, usize),        // Jump if register value is zero
    JumpIfNonZero(usize, usize),     // Jump if register value is non-zero
    Print(usize),                    // Print the value of a register
    Halt,                            // Halt execution
    AllocateMemory(usize),           // Allocate a block of memory
    FreeMemory(usize),               // Free allocated memory
    StoreToMemory(usize, usize, usize), // Store register value to memory
    LoadFromMemory(usize, usize),    // Load value from memory to register
    Call(usize),                     // Call a function at a specific offset
    Return,                          // Return from function
}
```

### `VM`

The `VM` struct represents the virtual machine and contains the state of the VM, including the instruction pointer, registers, memory, and other internal variables.

```
struct VM {
    ip: usize,                            // Instruction pointer
    program: Vec<Instruction>,            // The program instructions
    registers: Vec<i32>,                  // 8 registers for computation
    memory: HashMap<usize, MemoryRegion>, // Memory regions (mapped by address)
    next_free_address: usize,             // Tracks next free memory address
    stack: Vec<usize>,                    // Stack for function calls
}
```

## VM Methods

### `VM::new(program: Vec<Instruction>)`

Initializes a new virtual machine with the given program (a sequence of instructions).

### `VM::run(&mut self)`

Runs the program, executing each instruction sequentially until halted.

### Register Manipulation

*   **`set_reg(&mut self, register_index: usize, value: i32)`**: Sets the value of a specific register.

### Arithmetic and Comparison

*   **`add`, `sub`, `mul`, `div`, `mod_op`**: Perform arithmetic operations between two registers and store the result in a target register.
*   **`eq`, `neq`, `gt`, `lt`, `gte`, `lte`**: Perform comparison operations and store the result (1 for true, 0 for false) in a target register.

### Control Flow

*   **`jump(&mut self, pc_offset: usize)`**: Jumps to a specific instruction offset.
*   **`jump_if_zero`, `jump_if_non_zero`**: Conditional jumps based on the value of a register.

### Memory Management

*   **`allocate_memory(&mut self, size: usize)`**: Allocates a block of memory of the given size.
*   **`free_memory(&mut self, address: usize)`**: Frees a memory block at the specified address.
*   **`store_to_memory`, `load_from_memory`**: Store a value from a register into memory or load a value from memory into a register.

### Function Calls

*   **`call(&mut self, target_pc: usize)`**: Calls a function by jumping to the function’s address.
*   **`return_from_function(&mut self)`**: Returns from a function and continues execution from the saved return address.

### Printing

*   **`print(&self, register_index: usize)`**: Prints the value of the specified register.


## Instruction Set

Below is the list of instructions supported by the VM:

### Memory and I/O Operations:

*   `AllocateMemory(usize)`: Allocate a block of memory with the specified size.
*   `FreeMemory(usize)`: Free a previously allocated memory block.
*   `StoreToMemory(usize, usize, usize)`: Store the value of a register in memory at the specified address and offset.
*   `LoadFromMemory(usize, usize)`: Load a value from memory into a register.

### Arithmetic Operations:

*   `Add(usize, usize, usize)`: Add two registers and store the result in a target register.
*   `Sub(usize, usize, usize)`: Subtract one register from another and store the result in a target register.
*   `Mul(usize, usize, usize)`: Multiply two registers and store the result in a target register.
*   `Div(usize, usize, usize)`: Divide one register by another and store the result in a target register.
*   `Mod(usize, usize, usize)`: Compute the modulo of two registers and store the result in a target register.

### Comparison Operations:

*   `Eq(usize, usize, usize)`: Check if two registers are equal and store the result (1 for true, 0 for false).
*   `Neq(usize, usize, usize)`: Check if two registers are not equal and store the result.
*   `Gt(usize, usize, usize)`: Check if the first register is greater than the second and store the result.
*   `Lt(usize, usize, usize)`: Check if the first register is less than the second and store the result.
*   `Gte(usize, usize, usize)`: Check if the first register is greater than or equal to the second and store the result.
*   `Lte(usize, usize, usize)`: Check if the first register is less than or equal to the second and store the result.

### Control Flow:

*   `Jump(usize)`: Jump to a specific instruction offset.
*   `JumpIfZero(usize, usize)`: Jump to an instruction if the value of a register is zero.
*   `JumpIfNonZero(usize, usize)`: Jump if the value of a register is non-zero.
*   `Halt`: Stop the program execution.

### Function Calls:

*   `Call(usize)`: Call a function at a specific instruction pointer offset.
*   `Return`: Return from the function and continue execution from the return address.

### Register Operations:

*   `SetReg(usize, i32)`: Set a register to a specific value.
*   `Print(usize)`: Print the value of a register.


## Requirements

*install Rust

## Building

**Clone the repo**

```$ git clone https://github.com/tbruckschlegel/basic_rust_vm.git```

To run the code with debug output:
```
cargo build --features debug
cargo run --features debug
```
otherwise:
```
cargo build --release
cargo run --release
```


**Tests**
```
cargo test
```

## What's next?
```
* organize code into seperate files
* opcodes
* save/load support of programs
* basic compiler for the VM language
* non CPU execution target
```



