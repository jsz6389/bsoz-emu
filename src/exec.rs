/* SPDX-License-Identifier: MIT
 *
 * exec.rs
 *
 * Contains functions for executing instructions from memory
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */
use cpu;
use mem;


/*
 * Ties hex values to CPU instructions
 */
enum ins{
    LdaImmediate = 0xa9,
    LdaZeropage = 0xa5,
    LdaZeropageX = 0xb5,
    LdaAbsolute = 0xad,
    LdaAbsoluteY = 0xbd,
    LdaIndirectX = 0xa1,
    LdaIndirectY = 0xb1,

    BreakImplied = 0x00,

    JmpAbsolute = 0x4c,
    JmpIndirect = 0x6c,

    IncZeropage = 0xe6,
    IncZeropageX = 0xf6,
    IncAbsolute = 0xee,
    IncAbsoluteX = 0xfe,
    Inx = 0xe8,
    Iny = 0xc8
}


/*
 * Reads the next instruction from the program counter and executes it
 *
 * @param cpu the cpu on which the instruction will be executed
 *
 * @param mem the memory from which instructions will be executed
 *
 * @param cycles the number of cycles to be run
 */
pub fn exec(cpu: &mut cpu::Cpu, mem: &mut mem::Mem, mut cycles: usize){
    while(cycles > 0){

        // fetch the next instruction and increment the PC
        let ins:u8 = mem::fetch_byte(&mem, cpu.pc as usize);
        cpu.pc+=1;

        // Execute the instructions
        match ins {
            x if x == ins::BreakImplied as u8 => { cycles = 0; }

            _ => { println!("Invalid Instruction"); }
        }
    }
}
