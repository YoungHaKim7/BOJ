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

    let (d1, d2, d3) = (
        scan.token::<f64>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let a = (d1 + d2 - d3) / 2.0;
    let b = (d1 + d3 - d2) / 2.0;
    let c = (d2 + d3 - d1) / 2.0;

    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "1").unwrap();
        writeln!(out, "{:.1} {:.1} {:.1}", a, b, c).unwrap();
    }
}
