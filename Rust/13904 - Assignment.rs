use io::Write;
use std::{collections::BinaryHeap, io, str};

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
    let mut problems = vec![(0, 0); n];

    for i in 0..n {
        problems[i] = (scan.token::<usize>() - 1, scan.token::<i64>());
    }

    problems.sort();

    let mut priority_queue = BinaryHeap::new();
    let mut idx = n as i64 - 1;
    let mut ret = 0;

    for day in (0..1000).rev() {
        while idx >= 0 && problems[idx as usize].0 == day {
            priority_queue.push(problems[idx as usize].1);
            idx -= 1;
        }

        if !priority_queue.is_empty() {
            ret += priority_queue.pop().unwrap();
        }
    }

    writeln!(out, "{ret}").unwrap();
}
