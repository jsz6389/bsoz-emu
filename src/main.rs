/* SPDX-License-Identifier: MIT
 *
 * main.rs
 *
 * Functions for testing the 6502 CPU
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */
mod cpu;
mod mem;
mod exec;
mod address;
mod lda;


/*
 * Runs test on the cpu
 */
fn main() {
    // test the cpu
    let cpu = cpu::cpu_initialize();
    cpu::print_registers(&cpu);
    cpu::print_registers(&cpu);

    // test the mem
    let mut mem = mem::mem_initialize();

    // test reading and writing bytes
    mem::write_byte(&mut mem, 0x200, 69);
    let byte:u8 = mem::fetch_byte(&mem, 0x200);
    println!("The fetched byte was: {}", byte);

    // test reading and writing words
    mem::write_word(&mut mem, 0x300, 6769);
    let word:u16 = mem::fetch_word(&mem, 0x300);
    println!("The fetched word was: {}", word);
}
