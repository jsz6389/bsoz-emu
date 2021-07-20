/* SPDX-License-Identifier: MIT
 *
 * address.rs
 *
 * Contains functions for addressing memory
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
pub fn address_immediate(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = cpu.pc;
    cpu.pc+=1;

    return ptr;
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
pub fn address_zeropage(cpu: &mut cpu::Cpu, mem: &mem::Mem) -> u16 {
    let ptr:u16 = mem::fetch_byte(&mem, cpu.pc) as u16;
    cpu.pc+=1;

    return ptr;
}


