use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use std::convert::From;

pub struct Reader<R: Read> {
    buff: BufReader<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            buff: BufReader::new(inner),
        }
    }

    pub fn read_utf8_char(&mut self) -> io::Result<char> {
        let mut buff = [0; 4];
        self.read_exact(&mut buff[..1])?;
        let bytes_to_read = char_len(&buff)?;
        self.read_exact(&mut buff[1..bytes_to_read])?;
        from_bytes(&buff)
    }
}

impl<R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buff.read(buf)
    }
}

impl<R: Read> BufRead for Reader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.buff.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.buff.consume(amt)
    }
}

impl<R: Read + Seek> Seek for Reader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.buff.seek(pos)
    }
}

pub fn from_bytes(bytes: &[u8]) -> io::Result<char> {
    let len = char_len(bytes)?;
    if len == 1 {
        return Ok(char::from(bytes[0]));
    }
    verify_intermediate_bytes(&bytes[1..len])?;
    let cp = code_point(&bytes[..len]);
    println!(
        "Read bytes: {}, codepoint: {:X}",
        bytes_repr(&bytes[..]),
        cp
    );
    Ok(std::char::from_u32(cp).expect(&format!(
        "Verified utf-8 character could not be 
                parsed by the std library function: {}, codepoint {:X}",
        bytes_repr(&bytes[..]),
        cp
    )))
}

pub fn char_len(bytes: &[u8]) -> io::Result<usize> {
    assert!(bytes.len() > 0);
    let byte = bytes[0];
    if byte & 0b1000_0000 == 0b0000_0000 {
        return Ok(1);
    }
    if byte & 0b1110_0000 == 0b1100_0000 {
        return Ok(2);
    }
    if byte & 0b1111_0000 == 0b1110_0000 {
        return Ok(3);
    }
    if byte & 0b1111_1000 == 0b1111_0000 {
        return Ok(4);
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("Not an utf-8 character: {:X}", byte),
    ))
}

pub fn code_point(bytes: &[u8]) -> u32 {
    if bytes.len() == 1 {
        return bytes[0] as u32;
    }
    let mut cp = 0;
    for (i, byte) in bytes.iter().rev().enumerate() {
        cp = add_next_byte(cp, *byte, i as u32, bytes.len() as u32);
    }
    return cp;
}

fn verify_intermediate_bytes(bytes: &[u8]) -> io::Result<()> {
    for byte in bytes {
        if byte & 0b1100_0000 != 0b1000_0000 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Not an utf-8 character: {}", bytes_repr(bytes)),
            ));
        }
    }
    Ok(())
}

fn add_next_byte(cp: u32, byte: u8, num: u32, len: u32) -> u32 {
    if byte & 0b1100_0000 == 0b1000_0000 {
        add_intermediate_byte(cp, byte, num)
    } else {
        add_leading_byte(cp, byte, len)
    }
}

fn add_intermediate_byte(cp: u32, mut byte: u8, num: u32) -> u32 {
    byte &= 0b0011_1111;
    let byte = (byte as u32).overflowing_shl(num * 6).0;
    cp | byte
}

fn add_leading_byte(cp: u32, mut byte: u8, len: u32) -> u32 {
    byte &= leading_byte_clear_mask(len);
    let byte = (byte as u32).overflowing_shl((len - 1) * 6).0;
    cp | byte
}

fn leading_byte_clear_mask(len: u32) -> u8 {
    match len {
        2 => 0b0001_1111,
        3 => 0b0000_1111,
        4 => 0b0000_0111,
        _ => unreachable!(),
    }
}

fn bytes_repr(bytes: &[u8]) -> String {
    let mut repr = String::new();
    for byte in bytes {
        repr.extend(format!("{:X} ", byte).chars());
    }
    repr.pop();
    return repr;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_one_byte_utf8_character() {
        let mut reader = Reader::new("A".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), 'A');
    }

    #[test]
    fn read_two_byte_utf8_character() {
        let mut reader = Reader::new("ß".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), 'ß');
    }

    #[test]
    fn read_three_byte_utf8_character() {
        let mut reader = Reader::new("€".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), '€');
    }

    #[test]
    fn read_four_byte_utf8_character() {
        let mut reader = Reader::new("💣".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), '💣');
    }

    #[test]
    fn reading_multiple_utf8_characters() {
        let mut reader = Reader::new("💣ℝßA".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), '💣');
        assert_eq!(reader.read_utf8_char().unwrap(), 'ℝ');
        assert_eq!(reader.read_utf8_char().unwrap(), 'ß');
        assert_eq!(reader.read_utf8_char().unwrap(), 'A');
    }

    #[test]
    fn reading_past_returns_unexpected_eof() {
        let mut reader = Reader::new("".as_bytes());
        assert!(match reader.read_utf8_char() {
            Err(err) => match err.kind() {
                io::ErrorKind::UnexpectedEof => true,
                _ => false,
            },
            _ => false,
        });
    }
}
