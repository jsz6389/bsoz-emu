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
 * Fetches a pointer to the addressed data using immediate addressing
 *
 * @param cpu The cpu that will be addressing the pointer
 *
 * @param mem The memory that contains the desired data
 *
 * @return A pointer to the desired data
 */
pub fn address_immediate(cpu: &mut cpu::Cpu, _mem: &mem::Mem) -> u16 {
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
pub fn address_zeropage(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_zeropage_x(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_zeropage_y(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_relative(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_absolute(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_absolute_x(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_absolute_y(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_indirect(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
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
pub fn address_indexed_indirect(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, (mem::fetch_byte(&mem, cpu.pc.into()) + cpu.x).into());
    cpu.pc+=1;

    return 0;
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
pub fn address_indirect_indexed(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_word(&mem, mem::fetch_byte(&mem, cpu.pc as usize).into()) + cpu.y as u16;
    cpu.pc+=2;

    return ptr;
}
