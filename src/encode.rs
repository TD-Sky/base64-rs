use crate::alphabet::Alphabet;
use crate::U6;
use smallvec::{smallvec, SmallVec};

pub fn encode<T: Alphabet>(input: &[u8], alphabet: &T) -> String {
    input
        .chunks(3)
        .map(to_u6_chunk)
        .flat_map(|ck| encode_chunk(ck, alphabet))
        .collect()
}

fn to_u6_chunk(chunk: &[u8]) -> SmallVec<[U6; 4]> {
    match chunk.len() {
        3 => smallvec![
            chunk[0] >> 2,
            (chunk[0] & 0b00000011) << 4 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 2 | chunk[2] >> 6,
            chunk[2] & 0b00111111,
        ],

        2 => smallvec![
            chunk[0] >> 2,
            (chunk[0] & 0b00000011) << 4 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 2,
        ],

        1 => smallvec![chunk[0] >> 2, (chunk[0] & 0b00000011) << 4],

        _ => unreachable!(),
    }
}

fn encode_chunk<T: Alphabet>(u6_chunk: SmallVec<[U6; 4]>, alphabet: &T) -> [char; 4] {
    let mut chars = [<T as Alphabet>::PADDING; 4];

    for (i, &index) in u6_chunk.iter().enumerate() {
        chars[i] = alphabet.char(index);
    }

    chars
}
