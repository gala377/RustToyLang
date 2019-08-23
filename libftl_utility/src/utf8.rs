use std::io;
use std::io::{
    Read,
    BufReader,
    BufRead,
    Seek,
    SeekFrom,
};

use std::convert::From;

pub struct Reader<R: Read> { 
    buff: BufReader<R>,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self { buff: BufReader::new(inner) }
    }

    pub fn read_utf8_char(&mut self) -> io::Result<char> {
        // We won't expect characters longer than 4 bytes
        let mut buff = [0, 0, 0, 0];
        self.read_exact(&mut buff[..1])?;
        let bytes_to_read = Self::bytes_number(buff[0])?;
        if bytes_to_read == 1 {
            // We have already read 1 byte so we can now return it
            return Ok(char::from(buff[0]));
        }
        self.read_exact(&mut buff[1..bytes_to_read])?;
        Self::verify_bytes(&buff[1..bytes_to_read])?;
        let codepoint = u32::from_le_bytes([buff[0], buff[1], buff[2], buff[3]]);
        Ok(
            std::char::from_u32(codepoint).expect(
                &format!(
                    "Verified utf-8 character could not be 
                    parsed by the std library function: {}, codepoint {}",
                    Self::bytes_repr(&buff[..]),
                    codepoint))
        )
    }

    fn bytes_number(byte: u8) -> io::Result<usize> {
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
            format!("Not an utf-8 character: {:X}", byte))
        )
    }

    fn verify_bytes(bytes: &[u8]) -> io::Result<()> {
        for byte in bytes {
            if byte & 0b1100_0000 != 0b1000_0000 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Not an utf-8 character: {}",
                        Self::bytes_repr(bytes)))
                )
            }
        }
        Ok(())
    }

    fn bytes_repr(bytes: &[u8]) -> String {
        let mut repr = String::new();
        for byte in bytes {
            repr.extend(format!("{:X} ", byte).chars());
        }
        repr.pop();
        return repr;
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
        let mut reader = Reader::new("ℝ".as_bytes());
        assert_eq!(reader.read_utf8_char().unwrap(), 'ℝ');
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
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::UnexpectedEof => true,
                    _ => false,
                }
            }
            _ => false,
        });
    }
}