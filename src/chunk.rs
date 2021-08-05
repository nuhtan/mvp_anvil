use nbt::{Blob, Map, Value};

use crate::{block::Block, region::Region};

use std::{cmp, collections::HashMap, mem, ops::{Add, Sub}};

#[derive(Clone)]
pub struct Chunk {
    pub data: Box<Blob>,
}

impl Chunk {
    pub fn new(nbt_data: Box<Blob>) -> Chunk {
        let level_data = nbt_data;
        Chunk {
            data: level_data
        }
    }

    pub fn from_region(region: Region, chunk_x: u32, chunk_z: u32) -> Chunk {
        match region.chunk_data(chunk_x, chunk_z) {
            Some(data) => {
                let chunk = Chunk::new(data);
                return chunk
            },
            None => panic!("Got a none from chunk_data")
        }
    }

    // pub fn get_level_data(self) -> &'static HashMap<String, Value> {
    //     match self.data.get("Level").unwrap() {
    //         Value::Compound(level_data) => &level_data,
    //         _ => panic!("Got wrong data")
    //     }
    // }

    pub fn get_section(self, y: i8) -> Option<HashMap<String, Value>> {
        if y < -4 || y > 19 {
            panic!("Y value out of range")
        }
        let level_data = if let Value::Compound(c) = self.data.get("Level").unwrap() {
            c
        } else {
            panic!("Should be a compound")
        };
        let sections = if let Value::List(s) = level_data.get("Sections").unwrap() {
            s
        } else {
            panic!("Should be a list")
        };
        // let sections = self.get_level_data().get("Sections");
        for section in sections {
            let section = if let Value::Compound(s) = section {
                s
            } else {
                panic!("should be a compound")
            };
            let section_y = if let Value::Byte(sec_y) = section.get("Y").unwrap() {
                sec_y
            } else {
                panic!("Failed to get y")
            };
            if *section_y == y {
                let cloned = section.clone();
                return Some(cloned);
            }
        }
        None
    }

    pub fn get_block(self, x: i32, mut y: i32, z: i32) -> Block {
        let self_bits = self.clone();
        let section = self.get_section(((y + 64) / 16 - 4) as i8);
        if section == None {
            return Block::from_name(String::from("minecraft:air"));
        }
        let section = section.unwrap();
        y %= 16;
        let block_states = if let Some(Value::LongArray(bs)) = section.get("BlockStates") {
            Some(bs)
        } else {
            None
        };
        if block_states == None {
            return Block::from_name(String::from("minecraft:air"))
        }

        let palette = if let Value::List(p) = section.get("Palette").unwrap() {
            p
        } else {
            panic!("Palette should be a list")
        };
        let bits = cmp::max(self_bits.bit_length(palette.len() - 1), 4);
        let index = y * 16*16 + z * 16 + x;
        let states = block_states.unwrap();
        let state = index as usize / (64 / bits as usize);
        let mut data = states[state];
        if data < 0 {
            data += i64::MAX;
        }
        let shifted_data = data as usize >> (index as usize % (64 / bits as usize) * bits as usize);
        let palette_id = shifted_data & (2u32.pow(bits) - 1) as usize;
        let block = &palette[palette_id];
        return Block::from_palette(block)
    }

    pub fn bit_length(self, num: usize) -> u32 {
        // The number of bits that the number consists of, this is an integer and we don't care about signs or leading 0's
        // 0001 and 1 have the same return value
        // I think the lowest number that could come in is -1?
        // usize is always returned from the len function so I think that it will only be usize?
        if num == 0 {
            return 0
        }
        // Convert the number to a string version of the binary representation
        // Get the number of leading 0's
        let leading = num.leading_zeros();
        // Place the number into binary
        let s_num = format!("{:b}", num);
        // Remove leading 0's
        // let s = &s_num[leading as usize..];
        // Return the length
        return s_num.len() as u32
    }
}