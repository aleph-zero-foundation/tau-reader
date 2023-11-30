use crate::parameters::{CHALLENGE_HASH_SIZE, CHUNK_SIZE, G1_SIZE, G2_SIZE, NUMBER_OF_POWERS};
use crate::Chunk;

const CHUNKS: [Chunk; 7] = ["xaa", "xab", "xac", "xad", "xae", "xaf", "xag"];

fn coordinates(absolute_offset: usize) -> (Chunk, usize) {
    let chunk = CHUNKS[absolute_offset / CHUNK_SIZE];
    let inner_offset = absolute_offset % CHUNK_SIZE;
    (chunk, inner_offset)
}

/// Computes the position of the G1 point in the response file. Returns the chunk identifier and
/// the offset within the chunk.
///
/// Assumes that `index` is in bounds.
pub fn compute_g1_position(index: usize) -> (Chunk, usize) {
    coordinates(CHALLENGE_HASH_SIZE + index * G1_SIZE)
}

/// Computes the position of the G2 point in the response file. Returns the chunk identifier and
/// the offset within the chunk.
///
/// Assumes that `index` is in bounds.
pub fn compute_g2_position(index: usize) -> (Chunk, usize) {
    coordinates(CHALLENGE_HASH_SIZE + NUMBER_OF_POWERS * G1_SIZE + index * G2_SIZE)
}
