use crate::alphabet::Alphabet;
use crate::error::Error;

pub struct Standard;

impl Alphabet for Standard {
    const PADDING: char = '=';

    fn char(&self, index: u8) -> char {
        let index = index as i8;

        let ascii_index = match index {
            0..=25 => index + 65,  // A-Z
            26..=51 => index + 71, // a-z
            52..=61 => index - 4,  // 0-9
            62 => 43,              // +
            63 => 47,              // /
            _ => unreachable!(),
        } as u8;

        ascii_index as char
    }

    fn index(&self, char: char) -> Result<u8, Error> {
        let ascii_code = char as i8;
        let base64_index = match ascii_code {
            65..=90 => ascii_code - 65,  // A-Z
            97..=122 => ascii_code - 71, // a-z
            48..=57 => ascii_code + 4,   // 0-9
            43 => 62,                    // +
            47 => 63,                    // /
            _ => return Err(Error::InvalidChar(char)),
        } as u8;

        Ok(base64_index)
    }
}

mod tests {
    #[test]
    fn symmetry() {
        use super::Standard;

        let origin = b"runoob";
        let coding = "cnVub29i";

        assert_eq!(&crate::encode(origin, &Standard), coding);
        assert_eq!(
            crate::decode(coding, &Standard).as_deref(),
            Ok(origin.as_slice())
        );
    }
}
