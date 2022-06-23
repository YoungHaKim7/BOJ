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

fn calculate_max_square(num_square: &mut Vec<Vec<i64>>, n: usize, m: usize) -> i64 {
    if n % m == 0 {
        return n as i64 / m as i64;
    }

    if n == m {
        return 1;
    }

    if num_square[n][m] != -1 {
        return num_square[n][m];
    }

    num_square[n][m] = (n * m) as i64;

    if n >= 3 * m {
        num_square[n][m] = num_square[n][m].min(calculate_max_square(num_square, n - m, m) + 1);
    } else {
        for i in 1..=(n / 2) {
            num_square[n][m] = num_square[n][m].min(
                calculate_max_square(num_square, i, m) + calculate_max_square(num_square, n - i, m),
            );
        }

        for i in 1..=(m / 2) {
            num_square[n][m] = num_square[n][m].min(
                calculate_max_square(num_square, n, i) + calculate_max_square(num_square, n, m - i),
            );
        }
    }

    num_square[n][m]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut num_square = vec![vec![-1; m + 1]; n + 1];

    writeln!(out, "{}", calculate_max_square(&mut num_square, n, m)).unwrap();
}
