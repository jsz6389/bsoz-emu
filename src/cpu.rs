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
    
    pub pc: u16,    // Program Counter
    pub sp: u8,     // Stack Pointer
    pub a: u8,      // Accumulator
    pub x: u8,      // Index Register X
    pub y: u8,      // Index Register Y

    // Flags
    pub c: bool,    // Carry
    pub z: bool,    // Zero
    pub i: bool,    // Interrupt
    pub d: bool,    // Decimal
    pub b: bool,    // Break Command
    pub v: bool,    // Overflow
    pub n: bool,    // Negative
    pub cycles: i64
   
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
        c: true,
        z: true,
        i: true,
        d: true,
        b: true,
        v: true,
        n: true,
        cycles: 0xFFFFFFFF

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
    println!( "Flags" );
    println!( "Zero: {}", cpu.z );
    println!( "Carry: {}", cpu.c);
    println!( "Integer: {}", cpu.i );
    println!( "Decimal: {}", cpu.d );
    println!( "Break: {}", cpu.b );
    println!( "Overflow: {}", cpu.v );
    println!( "Negative: {}", cpu.n );
}
