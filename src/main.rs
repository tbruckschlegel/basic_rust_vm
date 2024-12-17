use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MemoryRegion {
    size: usize,
    data: Vec<u8>, // Data as raw bytes
}

#[derive(Debug)]
enum Instruction {
    SetReg(usize, i32),                 // Store value directly into a register
    Add(usize, usize, usize), // Add values from two registers and store in a target register
    Sub(usize, usize, usize), // Subtract values from two registers and store in a target register
    Mul(usize, usize, usize), // Multiply values from two registers and store in a target register
    Div(usize, usize, usize), // Divide values from two registers and store in a target register
    Mod(usize, usize, usize), // Modulo values from two registers and store in a target register
    Eq(usize, usize, usize), // Check equality of two registers and store result in a target register
    Neq(usize, usize, usize), // Check inequality of two registers and store result in a target register
    Gt(usize, usize, usize), // Check greater than between two registers and store result in a target register
    Lt(usize, usize, usize), // Check less than between two registers and store result in a target register
    Gte(usize, usize, usize), // Check greater than or equal to between two registers and store result in a target register
    Lte(usize, usize, usize), // Check less than or equal to between two registers and store result in a target register
    Jump(usize),              // Jump to a specific instruction offset
    JumpIfZero(usize, usize), // Jump if register value is zero
    JumpIfNonZero(usize, usize), // Jump if register value is non-zero
    Print(usize),             // Print the value of a register
    Halt,                     // Halt the execution
    AllocateMemory(usize),    // Allocate a memory block of a specific size
    FreeMemory(usize),        // Free a memory block
    StoreToMemory(usize, usize, usize), // Store a byte in memory at a specific address
    LoadFromMemory(usize, usize), // Load a byte from memory at a specific address
    Call(usize),              // Call a function at the specific PC offset
    Return,                   // Return from a function
}

struct VM {
    pc: usize,                            // Program counter
    program: Vec<Instruction>,            // The program instructions
    registers: Vec<i32>,                  // 8 registers
    memory: HashMap<usize, MemoryRegion>, // Memory regions
    next_free_address: usize,             // Tracks the next free address for allocation
    stack: Vec<usize>,                    // Stack for function call management (return addresses)
}

impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        VM {
            pc: 0,
            program,
            registers: vec![0; 8], // 8 registers initialized to zero
            memory: HashMap::new(),
            next_free_address: 0, // Initial free address is 0
            stack: Vec::new(),    // Stack for function calls
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            let instruction = &self.program[self.pc];
            self.pc += 1;

            match instruction {
                Instruction::SetReg(register_index, value) => {
                    self.set_reg(*register_index, *value);
                }
                Instruction::Add(register_a, register_b, target_register) => {
                    self.add(*register_a, *register_b, *target_register);
                }
                Instruction::Sub(register_a, register_b, target_register) => {
                    self.sub(*register_a, *register_b, *target_register);
                }
                Instruction::Mul(register_a, register_b, target_register) => {
                    self.mul(*register_a, *register_b, *target_register);
                }
                Instruction::Div(register_a, register_b, target_register) => {
                    self.div(*register_a, *register_b, *target_register);
                }
                Instruction::Mod(register_a, register_b, target_register) => {
                    self.mod_op(*register_a, *register_b, *target_register);
                }
                Instruction::Eq(register_a, register_b, target_register) => {
                    self.eq(*register_a, *register_b, *target_register);
                }
                Instruction::Neq(register_a, register_b, target_register) => {
                    self.neq(*register_a, *register_b, *target_register);
                }
                Instruction::Gt(register_a, register_b, target_register) => {
                    self.gt(*register_a, *register_b, *target_register);
                }
                Instruction::Lt(register_a, register_b, target_register) => {
                    self.lt(*register_a, *register_b, *target_register);
                }
                Instruction::Gte(register_a, register_b, target_register) => {
                    self.gte(*register_a, *register_b, *target_register);
                }
                Instruction::Lte(register_a, register_b, target_register) => {
                    self.lte(*register_a, *register_b, *target_register);
                }
                Instruction::Jump(pc_offset) => {
                    self.jump(*pc_offset);
                }
                Instruction::JumpIfZero(register_index, pc_offset) => {
                    self.jump_if_zero(*register_index, *pc_offset);
                }
                Instruction::JumpIfNonZero(register_index, pc_offset) => {
                    self.jump_if_non_zero(*register_index, *pc_offset);
                }
                Instruction::Print(register_index) => {
                    self.print(*register_index);
                }
                Instruction::Halt => break,
                Instruction::AllocateMemory(size) => {
                    self.allocate_memory(*size);
                }
                Instruction::FreeMemory(address) => {
                    self.free_memory(*address);
                }
                Instruction::StoreToMemory(address, register_index, offset) => {
                    self.store_to_memory(*address, *register_index, *offset);
                }
                Instruction::LoadFromMemory(address, register_index) => {
                    self.load_from_memory(*address, *register_index);
                }
                Instruction::Call(pc_offset) => {
                    self.call(*pc_offset);
                }
                Instruction::Return => {
                    self.return_from_function();
                }
            }
        }
    }

    fn set_reg(&mut self, register_index: usize, value: i32) {
        if register_index < self.registers.len() {
            self.registers[register_index] = value;
            println!("Set register {} to value {}", register_index, value);
        } else {
            println!("Error: Invalid register index.");
        }
    }

    fn add(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.binary_op(reg_a, reg_b, target_register, |a, b| a + b, "Add");
    }

    fn sub(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.binary_op(reg_a, reg_b, target_register, |a, b| a - b, "Sub");
    }

    fn mul(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.binary_op(reg_a, reg_b, target_register, |a, b| a * b, "Mul");
    }

    fn div(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.binary_op(reg_a, reg_b, target_register, |a, b| a / b, "Div");
    }

    fn mod_op(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.binary_op(reg_a, reg_b, target_register, |a, b| a % b, "Mod");
    }

    fn binary_op<F>(
        &mut self,
        reg_a: usize,
        reg_b: usize,
        target_register: usize,
        op: F,
        op_name: &str,
    ) where
        F: Fn(i32, i32) -> i32,
    {
        if reg_a < self.registers.len()
            && reg_b < self.registers.len()
            && target_register < self.registers.len()
        {
            let result = op(self.registers[reg_a], self.registers[reg_b]);
            self.registers[target_register] = result;
            println!(
                "{}: {} and {} -> {} (stored in register {})",
                op_name, self.registers[reg_a], self.registers[reg_b], result, target_register
            );
        } else {
            println!("Error: Invalid register index.");
        }
    }

    fn eq(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a == b, "Eq");
    }

    fn neq(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a != b, "Neq");
    }

    fn gt(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a > b, "Gt");
    }

    fn lt(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a < b, "Lt");
    }

    fn gte(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a >= b, "Gte");
    }

    fn lte(&mut self, reg_a: usize, reg_b: usize, target_register: usize) {
        self.compare_op(reg_a, reg_b, target_register, |a, b| a <= b, "Lte");
    }

    fn compare_op<F>(
        &mut self,
        reg_a: usize,
        reg_b: usize,
        target_register: usize,
        op: F,
        op_name: &str,
    ) where
        F: Fn(i32, i32) -> bool,
    {
        if reg_a < self.registers.len()
            && reg_b < self.registers.len()
            && target_register < self.registers.len()
        {
            let result = op(self.registers[reg_a], self.registers[reg_b]);
            self.registers[target_register] = if result { 1 } else { 0 };
            println!(
                "{}: {} and {} -> {} (stored in register {})",
                op_name,
                self.registers[reg_a],
                self.registers[reg_b],
                if result { 1 } else { 0 },
                target_register
            );
        } else {
            println!("Error: Invalid register index.");
        }
    }

    fn allocate_memory(&mut self, size: usize) {
        let address = self.next_free_address;
        self.memory.insert(
            address,
            MemoryRegion {
                size,
                data: vec![0; size],
            },
        );
        self.next_free_address += size;
        println!("Allocated {} bytes of memory at address {}", size, address);
    }

    fn free_memory(&mut self, address: usize) {
        if self.memory.remove(&address).is_some() {
            println!("Freed memory at address {}", address);
        } else {
            println!("Error: No memory block found at address {}", address);
        }
    }

    fn store_to_memory(&mut self, address: usize, register_index: usize, offset: usize) {
        if let Some(region) = self.memory.get_mut(&address) {
            if offset < region.size {
                region.data[offset] = self.registers[register_index] as u8;
                println!(
                    "Stored value {} from register {} at memory address {} and offset {}",
                    self.registers[register_index], register_index, address, offset
                );
            } else {
                println!("Error: Memory offset out of bounds.");
            }
        } else {
            println!("Error: No memory region found at address {}", address);
        }
    }

    fn load_from_memory(&mut self, address: usize, register_index: usize) {
        if let Some(region) = self.memory.get(&address) {
            let value = region.data[0] as i32; // For simplicity, just loading the first byte.
            self.registers[register_index] = value;
            println!(
                "Loaded value {} from memory address {} into register {}",
                value, address, register_index
            );
        } else {
            println!("Error: No memory region found at address {}", address);
        }
    }

    fn jump(&mut self, pc_offset: usize) {
        if self.pc + pc_offset < self.program.len() {
            self.pc += pc_offset;
            println!("Jumping to instruction {}", self.pc);
        } else {
            println!("Error: Invalid jump target.");
        }
    }

    fn jump_if_zero(&mut self, register_index: usize, pc_offset: usize) {
        if self.registers[register_index] == 0 {
            self.jump(pc_offset);
        }
    }

    fn jump_if_non_zero(&mut self, register_index: usize, pc_offset: usize) {
        if self.registers[register_index] != 0 {
            self.jump(pc_offset);
        }
    }

    fn print(&self, register_index: usize) {
        if register_index < self.registers.len() {
            println!(
                "Register {}: {}",
                register_index, self.registers[register_index]
            );
        } else {
            println!("Error: Invalid register index.");
        }
    }

    fn call(&mut self, target_pc: usize) {
        // Push the return address to the stack
        self.stack.push(self.pc);
        // Jump to the function address offset
        self.pc += target_pc;
        println!("Calling function at {}", self.pc);
    }

    fn return_from_function(&mut self) {
        // Pop the return address from the stack and continue
        if let Some(return_address) = self.stack.pop() {
            self.pc = return_address;
        }
    }
}

fn main() {
    let program = vec![
        Instruction::AllocateMemory(100),    // Allocate 100 bytes
        Instruction::SetReg(0, 42),          // Set reg0 to 42
        Instruction::StoreToMemory(0, 0, 0), // Store value in memory at address 0 and offset 0
        Instruction::LoadFromMemory(0, 1),   // Load value from memory at address 0 into reg1 1
        Instruction::Print(1),               // Print reg1
        Instruction::FreeMemory(0),          // Free memory at address 0
        Instruction::SetReg(0, 2),           // reg0 = 2
        Instruction::SetReg(1, 3),           // reg1 = 3
        Instruction::Call(5),                // Call function 1 (offset 6)
        Instruction::Print(0),               // Print reg0
        Instruction::JumpIfZero(4, 1),       // if reg4 is is 0, jump on instruction
        Instruction::Print(4),               // Print reg4
        Instruction::Print(2),               // Print reg2
        Instruction::Halt,                   // Halt the VM
        // function 1
        Instruction::SetReg(2, 11), // Store 11 in reg2
        Instruction::SetReg(3, 22), // Store 22 in reg3
        Instruction::Add(2, 3, 4),  // Add reg2 and reg3, result in reg4
        Instruction::Print(4),      // Print reg4
        Instruction::Return,        // Return from function
    ];

    let mut vm = VM::new(program);
    vm.run();
}