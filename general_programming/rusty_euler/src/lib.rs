use num::integer::Roots;

#[derive(Debug)]
pub struct TriangleNumbers {
    counter: u64,
    curr: u64,
}

impl Default for TriangleNumbers {
    fn default() -> Self {
        TriangleNumbers {counter: 1, curr: 0}
    }
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += self.counter; 
        self.counter += 1;
        Some(self.curr)
    }
}

#[derive(Debug)]
pub struct Fibonacci {
    a: i64,
    b: i64,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci { a: 1, b: 0 }
    }
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci::default()
    }
}

impl Iterator for Fibonacci {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let swap = self.b;
        self.b = self.a;
        self.a += swap;
        Some(self.b)
    }
}

pub struct Primes {
    curr: i64,
    next: i64,
}

impl Default for Primes {
    fn default() -> Self {
        Primes { curr: 1, next: 2 }
    }
}

impl Primes {
    pub fn new() -> Self {
        Primes::default()
    }

    fn is_prime(n: i64) -> bool {
        for i in 2..=n.sqrt() {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

impl Iterator for Primes {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let swap = self.next;
        let mut i = self.next + 1;

        while !Primes::is_prime(i) {
            i += 1;
        }

        self.next = i;
        self.curr = swap;
        Some(self.curr)
    }
}

pub enum GridDirection {
    up,
    down,
    left,
    right,
    upper_diagonal,
    lower_diagonal,
    upper_antidiagonal,
    lower_antidiagonal,
}

pub fn collect_line_from_grid(
    grid: &[Vec<i64>],
    mut row: usize,
    mut col: usize,
    length: usize,
    direction: GridDirection,
) -> Option<Vec<i64>> {

    let mut line: Vec<i64> = Vec::new();
    let row_dir: i64;
    let col_dir: i64;

    match direction {
        GridDirection::up => {
            row_dir = 0;
            col_dir = -1;
        },
        GridDirection::down => {
            row_dir = 0;
            col_dir = 1;
        },
        GridDirection::right => {
            row_dir = 1;
            col_dir = 0;
        },
        GridDirection::left => {
            row_dir = -1;
            col_dir = 0;
        },
        GridDirection::upper_diagonal => {
            row_dir = -1;
            col_dir = -1;
        },
        GridDirection::lower_diagonal => {
            row_dir = 1;
            col_dir = 1;
        },
        GridDirection::upper_antidiagonal => {
            row_dir = 1;
            col_dir = -1;
        },
        GridDirection::lower_antidiagonal => {
            row_dir = -1;
            col_dir = 1;
        }
    }
    for _ in 0..length {
        let current_cell = grid.get(row)?.get(col)?;
        line.push(current_cell.clone());
        if row_dir >= 0 {
            row = row.checked_add(row_dir as usize)?;
        } else {
            row = row.checked_sub(row_dir.unsigned_abs() as usize)?;
        }
        if col_dir >= 0 {
            col = col.checked_add(col_dir as usize)?;
        } else {
            col = col.checked_sub(col_dir.unsigned_abs() as usize)?;
        }
    }
    Some(line)
}

pub fn num_divisors(n: &u64) -> u64 {
    let mut cnt = 0;
    let sqrt = n.sqrt();
    for i in 1..sqrt {
        if n % i == 0 {
            cnt += 2;
        } 
    }
    if sqrt * sqrt == *n {
        cnt += 1;
    }
    cnt
}
