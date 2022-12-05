use crate::alphabet::Alphabet;
use crate::error::Error;
use smallvec::{smallvec, SmallVec};

pub fn decode<T: Alphabet>(input: &str, alphabet: &T) -> Result<Vec<u8>, Error> {
    if input.len() % 4 != 0 {
        return Err(Error::UnalignedStr);
    }

    input
        .chars()
        .array_chunks::<4>()
        .try_fold(Vec::new(), |mut acc, chunk| {
            acc.extend(to_u8_chunk(decode_chunk(chunk, alphabet)?));
            Ok(acc)
        })
}

fn decode_chunk<T: Alphabet>(chunk: [char; 4], alphabet: &T) -> Result<SmallVec<[u8; 4]>, Error> {
    chunk
        .into_iter()
        .take_while(|&c| c != <T as Alphabet>::PADDING)
        .map(|c| alphabet.index(c))
        .collect()
}

fn to_u8_chunk(chunk: SmallVec<[u8; 4]>) -> SmallVec<[u8; 3]> {
    match chunk.len() {
        4 => smallvec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4 | chunk[2] >> 2,
            (chunk[2] & 0b00000011) << 6 | chunk[3] & 0b00111111,
        ],

        3 => smallvec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4 | chunk[2] >> 2,
            (chunk[2] & 0b00000011) << 6,
        ],

        2 => smallvec![
            (chunk[0] & 0b00111111) << 2 | chunk[1] >> 4,
            (chunk[1] & 0b00001111) << 4,
        ],

        _ => unreachable!(),
    }
}
