/* SPDX-License-Identifier: MIT
 *
 * address.rs
 *
 * Contains functions for addressing memory
 * For more information on x86 addressing modes please see
 * http://obelisk.me.uk/6502/addressing.html
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */
use cpu;
use mem;


/*
 * 6502 addressing modes
 */
pub enum Addr {
    Imp,    //Implicit
    Acc,    //Accumulator
    Imm,    //Immediate
    Zero,   //Zeropage
    ZeroX,  //Zeropage,X
    ZeroY,  //Zeropage,Y
    Rel,    //Relative
    Abs,    //Absolute
    AbsX,   //Absolute,X
    AbsY,   //Absolute,Y
    Indir,  //Indirect
    DexDir, //Indexed Indirect
    DirDex  //Indirect Indexed
}


/*
 * Fetches a pointer to the addressed data using immediate addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn immediate(cpu: &mut cpu::Cpu, _mem: &mem::Mem) -> u16 {
    let ptr:u16 = cpu.pc;
    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the addressed data using zeropage addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn zeropage(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_byte(&mem, cpu.pc.into()) as u16;
    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the addressed data using zeropage,x addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn zeropage_x(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = (mem::fetch_byte(&mem, cpu.pc.into()) + cpu.x).into();
    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the address data using zeropage,y addressing
 *
 * @param cpu The cpu that will addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn zeropage_y(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = (mem::fetch_byte(&mem, cpu.pc.into()) + cpu.y).into();
    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the address data using relative addressing
 *
 * @param cpu The cpu that will addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn relative(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let mut byte:u8 = mem::fetch_byte(&mem, cpu.pc.into());
    let ptr:u16;

    if byte >= 128 {
        byte = !byte + 1;
        ptr = cpu.pc - byte as u16;
    } else {
        ptr = cpu.pc + byte as u16;
    }

    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the address data using absolute addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn absolute(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, cpu.pc.into());
    cpu.pc+=2;

    return ptr;
}


/*
 * Fetches a pointer to the address data using absolute,x addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn absolute_x(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, cpu.pc.into()) + cpu.x as u16;
    cpu.pc+=2;

    return ptr;
}


/*
 * Fetches a pointer to the addressed data using absolute,y addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn absolute_y(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, cpu.pc.into()) + cpu.y as u16;
    cpu.pc+=2;

    return ptr;
}


/*
 * Fetches a pointer to the address data using indirect addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn indirect(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, mem::fetch_word(&mem, cpu.pc.into()).into());
    cpu.pc+=2;

    return ptr;
}


/*
 * Fetches a pointer to the address data using indexed indirect addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn indexed_indirect(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, (mem::fetch_byte(&mem, cpu.pc.into()) + cpu.x).into());
    cpu.pc+=1;

    return ptr;
}


/*
 * Fetches a pointer to the address data using indirect indexed addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn indirect_indexed(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, mem::fetch_byte(&mem, cpu.pc as usize).into()) + cpu.y as u16;
    cpu.pc+=2;

    return ptr;
}
