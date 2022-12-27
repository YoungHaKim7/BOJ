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

    let n = scan.token::<usize>();
    let mut acrobats = vec![(0, 0); n];

    for i in 0..n {
        acrobats[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let weight_sum = acrobats.iter().map(|x| x.0).sum::<i64>();
    let mut acrobats_new = vec![(0, 0); n];

    for i in 0..n {
        acrobats_new[i] = (weight_sum - acrobats[i].0 - acrobats[i].1, i);
    }

    acrobats_new.sort();

    let mut weight_acc = 0;
    let mut ret = i64::MIN;

    for i in 0..n {
        ret = ret.max(acrobats_new[i].0 - weight_acc);
        weight_acc += acrobats[acrobats_new[i].1].0;
    }

    writeln!(out, "{ret}").unwrap();
}
