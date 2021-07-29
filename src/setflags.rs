/* SPDX-License-Identifier: MIT
 *
 * setflags.rs
 *
 * Contains functions for setting the cpu flags
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */
use cpu;


/*
 * Sets the zero flag based on the provided value
 *
 * @param cpu The cpu whose flag is to be modified
 *
 * @param val The value to be checked against
 *
 */
pub fn zero(cpu: &mut cpu::Cpu, val: u8) {

    cpu.z = val == 0 as u8;

}


/*
 * Sets the negative flag based on the provided value
 *
 * @param cpu The cpu whose flag is to be modified
 *
 * @param val The value to be checked against
 *
 */
pub fn negative(cpu: &mut cpu::Cpu, val: u8) {
    
    cpu.n = val|0b0111_1111 == 0b1111_1111;

}
