#![no_std]
#![feature(int_log)]

/// This is a very simple buffer to pre format a short line of text
/// limited arbitrarily to 16 bytes/characters => 128 bits.
pub struct FmtBuf {
    buf: [u8; 16],
    ptr: usize,
}

impl FmtBuf {
    pub fn new() -> Self {
        Self {
            buf: [0; 16],
            ptr: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[0..self.ptr]).unwrap()
    }
}

impl From<u16> for FmtBuf {
    fn from(num: u16) -> Self {
        let mut buf = FmtBuf::new();
        let mut number = num;

        if number > 9 {
            let amount_of_numbers = (number.log(10) as u8) + 1;
            for index in (0..amount_of_numbers).rev() {
                buf.buf[index as usize] = (number % 10) as u8 + 48;
                number /= 10;
            }
            buf.ptr += amount_of_numbers as usize;
        } else {
            buf.buf[0] = (number as u8) + 48;
            buf.ptr += 1;
        }

        buf
    }
}

impl core::fmt::Write for FmtBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let rest_len = self.buf.len() - self.ptr;
        
        let len = if rest_len < s.len() {
            rest_len
        } else {
            s.len()
        };
        
        self.buf[self.ptr..(self.ptr + len)].copy_from_slice(&s.as_bytes()[0..len]);
        self.ptr += len;
        
        Ok(())
    }
}

#[test]
pub fn test_runner() {
    use core::fmt::Write;

    let test_cases: &[(u16, &'static str)] = &[(2, "2"), (9, "9"), (375, "375"), (9999, "9999")];

    for (int_num, str_num) in test_cases {
        let num: u16 = *int_num;
        let num_buf = FmtBuf::from(num);
        let mut num_buf2 = FmtBuf::new();
        write!(num_buf2, "{}", str_num).unwrap();
        assert_eq!(num_buf.buf, num_buf2.buf);
    }
}
