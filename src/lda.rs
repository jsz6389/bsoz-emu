/* SPDX-License-Identifier: MIT
 *
 * lda.rs
 *
 * Contains functions for performing LDA instructions
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 */
use cpu;
use mem;
use address;
use setflags;


/*
 * Executes the LDA instruction
 *
 * @param cpu The cpu onto which the instruction will be executed
 *
 * @param mem The memory to be addressed
 *
 * @param addr The addressing mode to be used
 */
pub fn ins(cpu: &mut cpu::Cpu, mem: &mut mem::Mem, addr: address::Addr) {
    // fetch the proper pointer based on the addressing mode
    let ptr:u16;
    match addr {
        address::Addr::Imm => { ptr = address::immediate( cpu, mem); cpu.cycles-=2; }
        address::Addr::Zero => { ptr = address::zeropage( cpu, mem); cpu.cycles-=3; }
        address::Addr::ZeroX => { ptr = address::zeropage_x( cpu, mem); cpu.cycles-=4; }
        address::Addr::Abs => { ptr = address::absolute( cpu, mem); cpu.cycles-=4; }
        address::Addr::AbsY => { ptr = address::absolute_y( cpu, mem); cpu.cycles-=4; }
        address::Addr::DexDir => { ptr = address::indexed_indirect( cpu, mem); cpu.cycles-=6; }
        address::Addr::DirDex => { ptr = address::indirect_indexed( cpu, mem); cpu.cycles-=5; }
        _ => { println!("Invalid Addressing mode"); ptr = 0; }
    }

    cpu.a = mem::fetch_byte(&mem, ptr as usize); // set the accumulator to the addressed pointer
    
    let val = cpu.a;
    setflags::zero(cpu, val);
    let val = cpu.a;
    setflags::zero(cpu, val);
}
