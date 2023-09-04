use crate::constants::*;
use bevy::prelude::*;

// TODO: Move all three to world_map I believe
// Translates a block position into the chunk position of the chunk the block is in.
pub fn world_position_to_chunk_pos(mut position: IVec3) -> IVec3 {
    // Removing bits_of(chunk_size) - 1 is rounding down to nearest CHUNK_SIZE divisible.
    position = position & !(CHUNK_SIZE - 1) as i32;
    return position;
}

// Translates a block position into the index of the block in its chunk.
pub fn world_position_to_block_index(mut position: IVec3) -> usize {
    // Getting the last 4 bits will output 0->CHUNK_SIZE for both positive and negative numbers
    // because of two's complement.
    position = position & (CHUNK_SIZE - 1) as i32;
    return (position.x << 8 | position.z << 4 | position.y) as usize;
}

// Converts world space coordinates to index in self.chunks and index of block in chunk
pub fn world_position_to_chunk_position_and_block_index(position: IVec3) -> (IVec3, usize) {
    let chunk_coord = world_position_to_chunk_pos(position);
    let block_coord = world_position_to_block_index(position);

    return (chunk_coord, block_coord);
}

pub fn block_index_to_position(index: usize) -> IVec3 {
    const MASK: usize = CHUNK_SIZE - 1;
    let position = IVec3 {
        x: index as i32 >> 8,
        z: (index >> 4 & MASK) as i32,
        y: (index & MASK) as i32,
    };

    return position;
}
