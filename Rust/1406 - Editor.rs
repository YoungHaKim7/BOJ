use io::Write;
use std::{io, str};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for c in s.chars() {
        left.push(c as u8);
    }

    let m = scan.token::<i64>();

    for _ in 0..m {
        let command = scan.token::<String>();

        match command.as_str() {
            "L" => {
                if !left.is_empty() {
                    right.push(left.pop().unwrap());
                }
            }
            "D" => {
                if !right.is_empty() {
                    left.push(right.pop().unwrap());
                }
            }
            "B" => {
                if !left.is_empty() {
                    left.pop();
                }
            }
            "P" => {
                let c = scan.token::<String>();
                left.push(c.as_bytes()[0]);
            }
            _ => panic!("Unknown command"),
        }
    }

    while !left.is_empty() {
        right.push(left.pop().unwrap());
    }

    while !right.is_empty() {
        write!(out, "{}", right.pop().unwrap() as char).unwrap();
    }

    writeln!(out).unwrap();
}
