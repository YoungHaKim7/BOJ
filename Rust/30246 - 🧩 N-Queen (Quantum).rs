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

const EPS: f64 = 1e-8;
const INF: f64 = 1.0 / 0.0;

struct LPSolver {
    m: usize,
    n: usize,
    container_n: Vec<i64>,
    container_b: Vec<i64>,
    container_d: Vec<Vec<f64>>,
}

impl LPSolver {
    fn new(a: &Vec<Vec<f64>>, b: &Vec<f64>, c: &Vec<f64>) -> LPSolver {
        let m = b.len();
        let n = c.len();
        let mut container_n = vec![0; n + 1];
        let mut container_b = vec![0; m];
        let mut container_d = vec![vec![0.0; n + 2]; m + 2];

        for i in 0..m {
            for j in 0..n {
                container_d[i][j] = a[i][j];
            }
        }

        for i in 0..m {
            container_b[i] = (n + i) as i64;
            container_d[i][n] = -1.0;
            container_d[i][n + 1] = b[i];
        }

        for j in 0..n {
            container_n[j] = j as i64;
            container_d[m][j] = -c[j];
        }

        container_n[n] = -1;
        container_d[m + 1][n] = 1.0;

        LPSolver {
            m,
            n,
            container_n,
            container_b,
            container_d,
        }
    }

    fn pivot(&mut self, r: usize, s: usize) {
        let inv = 1.0 / self.container_d[r][s];

        for i in 0..self.m + 2 {
            if i != r && (self.container_d[i][s]).abs() > EPS {
                let inv2 = self.container_d[i][s] * inv;

                for j in 0..self.n + 2 {
                    self.container_d[i][j] -= self.container_d[r][j] * inv2;
                }

                self.container_d[i][s] = self.container_d[r][s] * inv2;
            }
        }

        for j in 0..self.n + 2 {
            if j != s {
                self.container_d[r][j] *= inv;
            }
        }

        for i in 0..self.m + 2 {
            if i != r {
                self.container_d[i][s] *= -inv;
            }
        }

        self.container_d[r][s] = inv;
        std::mem::swap(&mut self.container_b[r], &mut self.container_n[s]);
    }

    fn simplex(&mut self, phase: i64) -> bool {
        let x = self.m + phase as usize - 1;

        loop {
            let mut s = -1;

            for j in 0..self.n + 1 {
                if self.container_n[j] != -phase {
                    if s == -1
                        || (self.container_d[x][j], self.container_n[j])
                            < (
                                self.container_d[x][s as usize],
                                self.container_n[s as usize],
                            )
                    {
                        s = j as i64;
                    }
                }
            }

            if self.container_d[x][s as usize] >= -EPS {
                return true;
            }

            let mut r = -1;

            for i in 0..self.m {
                if self.container_d[i][s as usize] <= EPS {
                    continue;
                }

                if r == -1
                    || (
                        self.container_d[i][self.n + 1] / self.container_d[i][s as usize],
                        self.container_b[i],
                    ) < (
                        self.container_d[r as usize][self.n + 1]
                            / self.container_d[r as usize][s as usize],
                        self.container_b[r as usize],
                    )
                {
                    r = i as i64;
                }
            }

            if r == -1 {
                return false;
            }

            self.pivot(r as usize, s as usize);
        }
    }

    fn solve(&mut self, x: &mut Vec<f64>) -> f64 {
        let mut r = 0;

        for i in 1..self.m {
            if self.container_d[i][self.n + 1] < self.container_d[r][self.n + 1] {
                r = i;
            }
        }

        if self.container_d[r][self.n + 1] < -EPS {
            self.pivot(r, self.n);

            if !self.simplex(2) || self.container_d[self.m + 1][self.n + 1] < -EPS {
                return -INF;
            }

            for i in 0..self.m {
                if self.container_b[i] == -1 {
                    let mut s = 0;

                    for j in 1..self.n + 1 {
                        if self.container_n[j] != -1 {
                            if s == -1
                                || (self.container_d[i][j], self.container_n[j])
                                    < (
                                        self.container_d[i][s as usize],
                                        self.container_n[s as usize],
                                    )
                            {
                                s = j as i64;
                            }
                        }
                    }

                    self.pivot(i, s as usize);
                }
            }
        }

        if self.simplex(1) {
            for i in 0..self.m {
                if self.container_b[i] < self.n as i64 {
                    x[self.container_b[i] as usize] = self.container_d[i][self.n + 1];
                }
            }

            return self.container_d[self.m][self.n + 1];
        }
        INF
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut board = vec![vec![0.0; n]; n];

        for y in 0..n {
            for x in 0..n {
                board[y][x] = scan.token::<f64>();
            }
        }

        let size_total = n * n;
        let mut a = Vec::new();
        let mut b = Vec::new();
        let c = vec![0.0; size_total];

        for y in 0..n {
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            for x in 0..n {
                if board[y][x] == -1.0 {
                    val[y * n + x] = 1.0;
                } else {
                    rest -= board[y][x];
                }
            }

            a.push(val.clone());
            b.push(rest);

            for i in 0..size_total {
                val[i] *= -1.0;
            }

            a.push(val);
            b.push(-rest);
        }

        for x in 0..n {
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            for y in 0..n {
                if board[y][x] == -1.0 {
                    val[y * n + x] = 1.0;
                } else {
                    rest -= board[y][x];
                }
            }

            a.push(val.clone());
            b.push(rest);

            for i in 0..size_total {
                val[i] *= -1.0;
            }

            a.push(val);
            b.push(-rest);
        }

        let is_valid =
            |y: i64, x: i64| -> bool { y >= 0 && y < n as i64 && x >= 0 && x < n as i64 };

        for y in 0..n {
            let (mut y_curr, mut x_curr) = (y as i64, 0);
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            while is_valid(y_curr, x_curr) {
                if board[y_curr as usize][x_curr as usize] == -1.0 {
                    val[(y_curr * n as i64 + x_curr) as usize] = 1.0;
                } else {
                    rest -= board[y_curr as usize][x_curr as usize];
                }

                y_curr += 1;
                x_curr += 1;
            }

            a.push(val.clone());
            b.push(rest);
        }

        for x in 1..n {
            let (mut y_curr, mut x_curr) = (0, x as i64);
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            while is_valid(y_curr, x_curr) {
                if board[y_curr as usize][x_curr as usize] == -1.0 {
                    val[(y_curr * n as i64 + x_curr) as usize] = 1.0;
                } else {
                    rest -= board[y_curr as usize][x_curr as usize];
                }

                y_curr += 1;
                x_curr += 1;
            }

            a.push(val.clone());
            b.push(rest);
        }

        for y in 0..n {
            let (mut y_curr, mut x_curr) = (y as i64, 0);
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            while is_valid(y_curr, x_curr) {
                if board[y_curr as usize][x_curr as usize] == -1.0 {
                    val[(y_curr * n as i64 + x_curr) as usize] = 1.0;
                } else {
                    rest -= board[y_curr as usize][x_curr as usize];
                }

                y_curr -= 1;
                x_curr += 1;
            }

            a.push(val.clone());
            b.push(rest);
        }

        for x in 1..n {
            let (mut y_curr, mut x_curr) = (n as i64 - 1, x as i64);
            let mut val = vec![0.0; size_total];
            let mut rest = 1.0;

            while is_valid(y_curr, x_curr) {
                if board[y_curr as usize][x_curr as usize] == -1.0 {
                    val[(y_curr * n as i64 + x_curr) as usize] = 1.0;
                } else {
                    rest -= board[y_curr as usize][x_curr as usize];
                }

                y_curr -= 1;
                x_curr += 1;
            }

            a.push(val.clone());
            b.push(rest);
        }

        let mut result_board = vec![0.0; size_total];
        let ret = LPSolver::new(&a, &b, &c).solve(&mut result_board);

        if ret == -INF {
            writeln!(out, "NO").unwrap();
        } else {
            writeln!(out, "YES").unwrap();

            for y in 0..n {
                for x in 0..n {
                    if board[y][x] != -1.0 {
                        write!(out, "{:.9} ", board[y][x]).unwrap();
                    } else {
                        let mut e = result_board[y * n + x];

                        if e.abs() <= EPS {
                            e = 0.0;
                        }

                        write!(out, "{:.9} ", e).unwrap();
                    }
                }

                writeln!(out).unwrap();
            }
        }
    }
}
