use std::io::{BufRead, BufReader, Read};

pub trait ReadUtil {
    /// Reads a u8
    fn read_u8(&mut self) -> u8;
    /// Reads a u64
    fn read_u64(&mut self) -> u64;
    /// Reads a null terminated utf8 string
    fn read_utf8_nl_string(&mut self) -> String;
}

impl<R> ReadUtil for BufReader<R>
where
    R: Read,
{
    /// Reads a u8
    fn read_u8(&mut self) -> u8 {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer).expect("Unable to read u8.");
        u8::from_ne_bytes(buffer)
    }

    /// Reads a u64
    fn read_u64(&mut self) -> u64 {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer).expect("Unable to read u64.");
        u64::from_ne_bytes(buffer)
    }

    /// Reads a null terminated utf8 string
    fn read_utf8_nl_string(&mut self) -> String {
        let mut buffer = vec![];
        self.read_until(0u8, &mut buffer)
            .expect("peepeepoopoo this shouldn't fail");
        String::from_utf8(buffer)
            .expect("Unable to read valid utf8 string.")
            .trim_end_matches(0u8 as char)
            .to_owned()
    }
}
