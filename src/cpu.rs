/* SPDX-License-Identifier: MIT
 *
 * cpu.rs
 *
 * Contains structs and functions that represent the cpu
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */


/*
 * Contains all cpu registers
 */
pub struct Cpu{
    
    pc: u16,    // Program Counter
    sp: u8,     // Stack Pointer
    a: u8,      // Accumulator
    x: u8,      // Index Register X
    y: u8,      // Index Register Y
    flags: u8   // Flags
   
}


/*
 * Initializes the CPU and sets initial register values
 *
 * @Return The newly initialized CPU
 *
 */
pub fn cpu_initialize() -> Cpu {
    Cpu {

        pc: 0x200, // First 512 bytes reserved for page and stack
        sp: 0x00,
        a: 0x00,
        x: 0x00,
        y: 0x00,
        flags: 0x00

    }
}


/*
 * Prints all CPU registers and flag values
 */
pub fn print_registers(cpu: &Cpu) {
    println!( "PC: 0x{:X}", cpu.pc );
    println!( "SP: 0x{:X}", cpu.sp );
    println!( "A: 0x{:X}", cpu.a );
    println!( "X: 0x{:X}", cpu.x );
    println!( "Y: 0x{:X}", cpu.y );
    println!( "Flags: {}", cpu.flags );
}
