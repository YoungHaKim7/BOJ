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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut plants = vec![[-1, -1]; n + 1];

    for _ in 0..m {
        let (a, b, c) = (
            scan.token::<usize>(),
            scan.token::<char>(),
            scan.token::<i64>(),
        );

        if b == 'P' {
            plants[a][0] = c;
        } else {
            plants[a][1] = c;
        }
    }

    let min = plants
        .iter()
        .skip(1)
        .filter(|x| x[0] == 1 && x[1] == 0)
        .count();
    let max = plants
        .iter()
        .skip(1)
        .filter(|x| {
            (x[0] == 1 && x[1] == 0)
                || (x[0] == 1 && x[1] == -1)
                || (x[0] == -1 && x[1] == 0)
                || (x[0] == -1 && x[1] == -1)
        })
        .count();

    writeln!(out, "{min} {max}").unwrap();
}
