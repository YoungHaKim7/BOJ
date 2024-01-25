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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;

    loop {
        let (n, m) = (scan.token::<i64>() + 1, scan.token::<i64>() + 1);

        if n == 1 && m == 1 {
            break;
        }

        let total = n * m;
        let mut ret = total * (total - 1) * (total - 2) / 6;
        ret -= m * n * (n - 1) * (n - 2) / 6;
        ret -= n * m * (m - 1) * (m - 2) / 6;

        for i in 1..n {
            for j in 1..m {
                let gcd = gcd(i, j);

                if gcd != 1 {
                    ret -= 2 * (gcd - 1) * (n - i) * (m - j);
                }
            }
        }

        writeln!(out, "Case {t}: {ret}").unwrap();

        t += 1;
    }
}
