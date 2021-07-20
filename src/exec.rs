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
use address;
use lda;



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
    while cycles > 0 {

        // fetch the next instruction and increment the PC
        let ins:u8 = mem::fetch_byte(&mem, cpu.pc as usize);
        cpu.pc+=1;

        // Execute the instructions
        match ins {

            0xA9 => { lda::ins( cpu, mem, address::Addr::Imm ); } // LDA Immediate
            0xA5 => { lda::ins( cpu, mem, address::Addr::Zero ); } // LDA Zeropage
            0xB5 => { lda::ins( cpu, mem, address::Addr::ZeroX ); } // LDA Zeropage,X
            0xAD => { lda::ins( cpu, mem, address::Addr::Abs ); } // LDA Absolute
            0xBD => { lda::ins( cpu, mem, address::Addr::AbsY ); } // LDA Absolute,Y
            0xA1 => { lda::ins( cpu, mem, address::Addr::DexDir ); } // LDA Indexed Indirect
            0xB1 => { lda::ins( cpu, mem, address::Addr::DirDex ); } // LDA Indirect Indexed

            0x00 => { cycles = 0; } // Break Implied

            _ => { println!("Invalid Instruction"); }
        }
    }
}
