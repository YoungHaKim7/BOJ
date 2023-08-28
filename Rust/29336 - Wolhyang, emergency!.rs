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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut powers = vec![0; n];
    let mut conditions = vec![(0, 0); m];

    for i in 0..n {
        powers[i] = scan.token::<i64>();
    }

    for i in 0..m {
        conditions[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    powers.sort_by(|a, b| b.cmp(a));

    let mut ret = 0;
    let mut idx = 0;

    for i in 0..m {
        let (t, q) = conditions[i];

        while q > ret {
            if idx == n {
                writeln!(out, "-1").unwrap();
                return;
            }
            
            ret += powers[idx] + t;
            idx += 1;
        }

        if i == m - 1 {
            while idx < n {
                ret += powers[idx] + t;
                idx += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
