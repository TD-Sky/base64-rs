use crate::alphabet::Alphabet;
use crate::error::Error;
use crate::U6;
use smallvec::{smallvec, SmallVec};

pub fn decode<T: Alphabet>(input: &str, alphabet: &T) -> Result<Vec<u8>, Error> {
    // 使用码表编码以后，字节数即字符数
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

fn decode_chunk<T: Alphabet>(chunk: [char; 4], alphabet: &T) -> Result<SmallVec<[U6; 4]>, Error> {
    chunk
        .into_iter()
        .take_while(|&c| c != <T as Alphabet>::PADDING)
        .map(|c| alphabet.index(c))
        .collect()
}

/// 为什么不会出现只有一个字符的情况：
/// <https://en.wikipedia.org/wiki/Base64#Examples>
fn to_u8_chunk(chunk: SmallVec<[U6; 4]>) -> SmallVec<[u8; 3]> {
    match chunk.len() {
        4 => smallvec![
            stitch_first_second(chunk[0], chunk[1]),
            stitch_second_third(chunk[1], chunk[2]),
            (chunk[2] & 0b00000011) << 6 | (chunk[3] & 0b00111111),
        ],

        3 => smallvec![
            stitch_first_second(chunk[0], chunk[1]),
            stitch_second_third(chunk[1], chunk[2]),
            (chunk[2] & 0b00000011) << 6,
        ],

        2 => smallvec![
            stitch_first_second(chunk[0], chunk[1]),
            (chunk[1] & 0b00001111) << 4,
        ],

        _ => unreachable!(),
    }
}

#[inline]
fn stitch_first_second(first: U6, second: U6) -> u8 {
    (first & 0b00111111) << 2 | second >> 4
}

#[inline]
fn stitch_second_third(second: U6, third: U6) -> u8 {
    (second & 0b00001111) << 4 | third >> 2
}
