#![no_std]

/// This is a very simple buffer to pre format a short line of text
/// limited arbitrarily to 5 bytes/characters => 40 bits.
/// Reason why it only has 5 bytes, is because it converts u16 integer and
/// the maximum u16 is 65536(5 bytes needed) to represent
pub struct FmtBuf {
    buf: [u8; 5],
    ptr: usize,
}

impl FmtBuf {
    pub fn new() -> Self {
        Self {
            buf: [0; 5],
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

        while number > 0 {
            buf.buf[buf.ptr] = (number % 10) as u8 + 48;
            number /= 10;
            buf.ptr += 1;
        }

        if buf.ptr > 1 {
            let len = buf.ptr - 1;
            let half_len = buf.ptr / 2;
            for index in 0..half_len {
                let element_swapped = buf.buf[index];
                buf.buf[index] = buf.buf[len - index];
                buf.buf[len - index] = element_swapped;
            }
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

    let test_cases: &[(u16, &'static str)] = &[
        (2, "2"),
        (9, "9"),
        (10, "10"),
        (375, "375"),
        (1000, "1000"),
        (9999, "9999"),
        (16798, "16798"),
    ];

    for (int_num, str_num) in test_cases {
        let num: u16 = *int_num;
        let num_buf = FmtBuf::from(num);
        let mut num_buf2 = FmtBuf::new();
        write!(num_buf2, "{}", str_num).unwrap();
        assert_eq!(num_buf.buf, num_buf2.buf);
        assert_eq!(num_buf.ptr, num_buf2.ptr);
    }
}
