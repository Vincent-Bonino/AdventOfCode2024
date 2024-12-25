use std::cmp::min;
use std::fmt::{Debug, Formatter};

const MAX_REVERSE_SHIFT: u32 = 1000; // Because X = Y >> Z, assume Z in [0, 7]

#[derive(Clone, Debug)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    pub fn from_opcode(op_code: u8) -> Self {
        match op_code {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("Invalid opcode '{op_code}"),
        }
    }
}

#[derive(Clone, Default)]
pub struct Computer {
    // Registers
    pub a: u32,
    pub b: u32,
    pub c: u32,

    // Stack
    jump_flag: bool,
    pub stack: Vec<u8>,
    stack_pointer: usize,

    pub stdout: Vec<u8>,
}

impl Computer {
    pub fn new(a: u32, b: u32, c: u32, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            jump_flag: true,
            stack: program,
            stack_pointer: 0,
            stdout: Vec::new(),
        }
    }

    // Run methods

    pub fn run(&mut self) -> Vec<u8> {
        while self.stack_pointer < self.stack.len() {
            // Reset jump flag
            self.jump_flag = true;

            // Run next instruction
            let instruction: Instruction = Instruction::from_opcode(self.stack[self.stack_pointer]);
            let operand: u8 = self.stack[self.stack_pointer + 1];
            self.run_instruction(&instruction, operand);

            // Continue to next instruction
            if self.jump_flag {
                self.stack_pointer += 2
            }
        }

        self.stdout.clone()
    }

    pub fn format_stdout(stdout: &[u8]) -> String {
        let str_stdout: Vec<String> = stdout.iter().map(|val| val.to_string()).collect();
        str_stdout.join(",")
    }

    // Decompiler methods

    #[allow(dead_code)]
    pub fn decompile(&self) {
        println!(">> {:?}", self.stack);
        println!("START");
        for pointer in 0..(self.stack.len() / 2) {
            let instruction: Instruction = Instruction::from_opcode(self.stack[2 * pointer]);
            let operand: u8 = self.stack[2 * pointer + 1];

            match instruction {
                Instruction::Adv => println!("A = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Bxl => println!("B = B ^ {operand}"),
                Instruction::Bst => println!("B = {} & 7", Computer::decompile_combo(&operand)),
                Instruction::Jnz => println!("<jump to {operand}>",),
                Instruction::Bxc => println!("B = B ^ C"),
                Instruction::Out => println!("print({})", Computer::decompile_combo(&operand)),
                Instruction::Bdv => println!("B = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Cdv => println!("C = A >> {}", Computer::decompile_combo(&operand)),
            };
        }
        println!("END");
    }

    #[allow(dead_code)]
    fn decompile_combo(combo: &u8) -> String {
        match combo {
            0..=3 => combo.to_string(),
            4 => String::from("A"),
            5 => String::from("B"),
            6 => String::from("C"),
            7 => unimplemented!("Combo code 7 is reserved"),
            _ => unreachable!("Invalid combo value '{combo}'"),
        }
    }

    // Instructions

    fn run_instruction(&mut self, instruction: &Instruction, operand: u8) {
        match instruction {
            Instruction::Adv => self.run_adv(operand),
            Instruction::Bxl => self.run_bxl(operand),
            Instruction::Bst => self.run_bst(operand),
            Instruction::Jnz => self.run_jnz(operand),
            Instruction::Bxc => self.run_bxc(operand),
            Instruction::Out => self.run_out(operand),
            Instruction::Bdv => self.run_bdv(operand),
            Instruction::Cdv => self.run_cdv(operand),
        }
    }

    fn resolve_combo(&self, combo: u8) -> u32 {
        match combo {
            0..=3 => combo as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unimplemented!("Combo code 7 is reserved"),
            _ => unreachable!("Invalid combo value '{combo}'"),
        }
    }

    // Normal operations

    /// Division (opcode=0)
    #[inline]
    fn run_adv(&mut self, operand: u8) {
        self.a >>= self.resolve_combo(operand)
    }

    /// Bitwise XOR (opcode=1)
    #[inline]
    fn run_bxl(&mut self, operand: u8) {
        self.b ^= operand as u32
    }

    /// Modulo (opcode=2)
    #[inline]
    fn run_bst(&mut self, operand: u8) {
        self.b = self.resolve_combo(operand) & 7
    }

    /// Jump (opcode=3)
    #[inline]
    fn run_jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.stack_pointer = operand as usize;

            // Do not move stack pointer after jump
            self.jump_flag = false;
        }
    }

    /// Bitwise XOR (opcode=4)
    #[inline]
    fn run_bxc(&mut self, _operand: u8) {
        self.b ^= self.c
    }

    /// Write to stdout (opcode=5)
    #[inline]
    fn run_out(&mut self, operand: u8) {
        self.stdout.push((self.resolve_combo(operand) & 7) as u8);
    }

    /// Division (opcode=6)
    #[inline]
    fn run_bdv(&mut self, operand: u8) {
        self.b = self.a >> self.resolve_combo(operand)
    }

    /// Division (opcode=7)
    #[inline]
    fn run_cdv(&mut self, operand: u8) {
        self.c = self.a >> self.resolve_combo(operand)
    }

    // Reverse

    pub fn clone_for_reverse(&self, a: u32, b: u32, c: u32) -> Self {
        Self {
            a,
            b,
            c,
            jump_flag: false, // No care
            stack: self.stack.clone(),
            stack_pointer: self.stack_pointer,
            stdout: self.stdout.clone(),
        }
    }

    /// Run a single reversed operation
    ///
    /// Returns None if changes could be done in place.
    /// Otherwise, return all possible memories (not self alteration)
    pub fn run_reverse_operation(&mut self) -> Option<Vec<Self>> {
        // Move stack pointer, going back to the end if at the beginning
        if self.stack_pointer == 0 {
            self.stack_pointer = self.stack.len() - 2;
        } else {
            self.stack_pointer -= 2
        }

        // Run pointed instruction
        let instruction: Instruction = Instruction::from_opcode(self.stack[self.stack_pointer]);
        let operand: u8 = self.stack[self.stack_pointer + 1];

        match instruction {
            Instruction::Adv => self.run_reversed_adv(operand),
            Instruction::Bxl => self.run_reversed_bxl(operand),
            Instruction::Bst => self.run_reversed_bst(operand),
            Instruction::Jnz => self.run_reversed_jnz(operand),
            Instruction::Bxc => self.run_reversed_bxc(operand),
            Instruction::Out => self.run_reversed_out(operand),
            Instruction::Bdv => self.run_reversed_bdv(operand),
            Instruction::Cdv => self.run_reversed_cdv(operand),
        }
    }

    // Reversed operations

    /// Division (opcode=0)
    #[inline]
    fn run_reversed_adv(&mut self, operand: u8) -> Option<Vec<Self>> {
        let combo: u32 = self.resolve_combo(operand);
        let unknown_values_count: u32 = 2_u32.pow(combo);

        let new_a: u32 = self.a << combo;

        let mut possibilities: Vec<Self> = Vec::with_capacity(unknown_values_count as usize);
        for unknown_bits in 0..unknown_values_count {
            possibilities.push(
                self.clone_for_reverse(new_a + unknown_bits, self.b, self.c)
            );
        }

        Some(possibilities)
    }

    /// Bitwise XOR (opcode=1)
    #[inline]
    fn run_reversed_bxl(&mut self, operand: u8) -> Option<Vec<Self>> {
        self.b ^= operand as u32;
        None
    }

    /// Modulo (opcode=2)
    #[inline]
    fn run_reversed_bst(&mut self, operand: u8) -> Option<Vec<Self>> {
        // Let's say that only the X next bytes matter
        let arbitrary_choice: u32 = 3;

        let combo: u32 = self.resolve_combo(operand);
        let unknown_values_count: u32 = 2_u32.pow(arbitrary_choice);

        let mut possibilities: Vec<Self> = Vec::with_capacity(unknown_values_count as usize);
        for unknown_bits in 0..unknown_values_count {
            possibilities.push(
                self.clone_for_reverse(self.a, combo + unknown_bits << arbitrary_choice, self.c)
            );
        }

        Some(possibilities)
        // self.b = self.resolve_combo(operand) & 7
    }

    /// Jump (opcode=3)
    #[inline]
    fn run_reversed_jnz(&mut self, _operand: u8) -> Option<Vec<Self>> {
        None
    }

    /// Bitwise XOR (opcode=4)
    #[inline]
    fn run_reversed_bxc(&mut self, _operand: u8) -> Option<Vec<Self>> {
        self.b ^= self.c;
        None
    }

    /// Write to stdout (opcode=5)
    #[inline]
    fn run_reversed_out(&mut self, operand: u8) -> Option<Vec<Self>> {
        self.stdout.push((self.resolve_combo(operand) & 7) as u8);
        None
    }

    /// Division (opcode=6)
    #[inline]
    fn run_reversed_bdv(&mut self, operand: u8) -> Option<Vec<Self>> {
        let combo: u32 = self.resolve_combo(operand);
        let unknown_values_count: u32 = 2_u32.pow(combo);

        let new_a: u32 = self.a << combo;

        let mut possibilities: Vec<Self> = Vec::with_capacity(unknown_values_count as usize);
        for unknown_bits in 0..unknown_values_count {
            possibilities.push(
                self.clone_for_reverse(self.a, new_a + unknown_bits, self.c)
            );
        }

        Some(possibilities)
    }

    /// Division (opcode=7)
    #[inline]
    fn run_reversed_cdv(&mut self, operand: u8) -> Option<Vec<Self>> {
        let combo: u32 = self.resolve_combo(operand);
        let unknown_values_count: u32 = 2_u32.pow(combo);

        let new_a: u32 = self.a << combo;

        let mut possibilities: Vec<Self> = Vec::with_capacity(unknown_values_count as usize);
        for unknown_bits in 0..unknown_values_count {
            possibilities.push(
                self.clone_for_reverse(self.a, self.b, new_a + unknown_bits)
            );
        }

        Some(possibilities)
    }

    // Utils

    #[allow(dead_code)]
    pub fn show_next_reverse_operation(&self) {
        if self.stack_pointer == 0 {
            println!("At the start!")
        } else {
            let pointer = self.stack_pointer - 2;

            let instruction: Instruction = Instruction::from_opcode(self.stack[pointer]);
            let operand: u8 = self.stack[pointer + 1];

            match instruction {
                Instruction::Adv => println!("A = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Bxl => println!("B = B ^ {operand}"),
                Instruction::Bst => println!("B = {} & 7", Computer::decompile_combo(&operand)),
                Instruction::Jnz => println!("<jump to {operand}>", ),
                Instruction::Bxc => println!("B = B ^ C"),
                Instruction::Out => println!("print({})", Computer::decompile_combo(&operand)),
                Instruction::Bdv => println!("B = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Cdv => println!("C = A >> {}", Computer::decompile_combo(&operand)),
            };
        }
    }
}

impl Debug for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Computer(a={},b={},c={},stdout='{}',sp={})",
            self.a, self.b, self.c, Computer::format_stdout(&self.stdout), self.stack_pointer
        )
    }
}
