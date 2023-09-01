use io::Write;
use std::{collections::VecDeque, io, str};

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

fn process_bfs(grid: &Vec<Vec<i64>>, n: usize, amount_rain: i64) -> i64 {
    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; n]; n];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 || visited[i][j] || grid[i][j] <= amount_rain {
                continue;
            }

            queue.push_back((i, j));
            visited[i][j] = true;
            ret += 1;

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                for k in 0..4 {
                    let next_x = x as i64 + dx[k];
                    let next_y = y as i64 + dy[k];

                    if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= n as i64 {
                        continue;
                    }

                    let next_x = next_x as usize;
                    let next_y = next_y as usize;

                    if grid[next_x][next_y] == 0
                        || visited[next_x][next_y]
                        || grid[next_x][next_y] <= amount_rain
                    {
                        continue;
                    }

                    visited[next_x][next_y] = true;
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut grid = vec![vec![0; n]; n];
    let mut height_max = 0;

    for i in 0..n {
        for j in 0..n {
            grid[i][j] = scan.token::<i64>();
            height_max = height_max.max(grid[i][j]);
        }
    }

    let mut ret = 1;

    for i in 1..height_max {
        ret = ret.max(process_bfs(&grid, n, i));
    }

    writeln!(out, "{ret}").unwrap();
}
