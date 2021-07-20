/* SPDX-License-Identifier: MIT
 *
 * mem.rs
 *
 * Contains structs and functions representing memory
 *
 * Copyright (C) 2021 Jakob Zielinski <jakobzee3@gmail.com>
 *
 */


/*
 * The memory
 */
pub struct Mem{

    pub data: [u8; 0xFFFF]

}


/*
 * Initializes the memory
 *
 * @Return The newly initialized memory
 *
 */
pub fn mem_initialize() -> Mem {
    Mem {

        data: [0x00; 0xFFFF]

    }
}


/*
 * Writes a byte to memory
 *
 * @param mem The memory to be written to
 *
 * @param ptr The pointer in memory to be addressed
 *
 * @param data The byte to be written at the pointer
 *
 */
pub fn write_byte(mem: &mut Mem, ptr: usize, data: u8) {

    mem.data[ptr] = data;

}


/*
 * Writes a word to memory
 *
 * @param mem The memory to be written to
 *
 * @param ptr The pointer in memory to be addressed
 *
 * @param data The word to be written at the pointer
 *
 */
pub fn write_word(mem: &mut Mem, ptr: usize, data: u16) {
    let bytes = data.to_be_bytes(); // convert the data into an array of bytes

    mem.data[ptr] = bytes[0];
    mem.data[ptr+1] = bytes[1];

}


/*
 * Fetches a byte from memory
 *
 * @param mem The memory to be addressed
 *
 * @param ptr The pointer in memory to be fetched
 *
 * @return The fetched byte
 */
pub fn fetch_byte(mem: &Mem, ptr: usize) -> u8 {
    return mem.data[ptr];
}


/*
 * Fetches a word from memory
 *
 * @param mem The memory to be addressed
 *
 * @param ptr The pointer in memory to be fetched
 *
 * @return The fetched word
 */
pub fn fetch_word(mem: &Mem, ptr: usize) -> u16 {
    let mut word:u16 = mem.data[ptr].into(); //load the first byte
    word = word << 8; //put the first byte in its place
    word += mem.data[ptr+1] as u16; //load the second byte

    return word;
}

