use rand::Rng;
use std::env;

pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        
        WeightedQuickUnionUF {
            parent,
            size,
            count: n,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn validate(&self, p: usize) {
        let n = self.parent.len();
        if p >= n {
            panic!("index {} is not between 0 and {}", p, n-1);
        }
    }

    pub fn find(&self, mut p: usize) -> usize {
        self.validate(p);
        while p != self.parent[p] {
            p = self.parent[p];
        }
        p
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);
        if root_p == root_q {
            return;
        }

        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }
        self.count -= 1;
    }
}

struct Percolation {
    grid: Vec<Vec<bool>>,
    size: usize,
    uf: WeightedQuickUnionUF,
}

impl Percolation {
    fn new(n: usize) -> Percolation {
        Percolation {
            grid: vec![vec![false; n]; n],
            size: n,
            uf: WeightedQuickUnionUF::new(n * n + 2),
        }
    }

    fn open(&mut self, i: usize, j: usize) {
        if self.is_valid_index(i, j) {
            self.grid[i][j] = true;
            let current = i * self.size + j;
            if i == 0 {
                self.uf.union(current, self.size * self.size);
            }
            if i == self.size - 1 {
                self.uf.union(current, self.size * self.size + 1);
            }
            if i > 0 && self.is_open(i - 1, j) {
                self.uf.union(current, (i - 1) * self.size + j);
            }
            if i < self.size - 1 && self.is_open(i + 1, j) {
                self.uf.union(current, (i + 1) * self.size + j);
            }
            if j > 0 && self.is_open(i, j - 1) {
                self.uf.union(current, i * self.size + j - 1);
            }
            if j < self.size - 1 && self.is_open(i, j + 1) {
                self.uf.union(current, i * self.size + j + 1);
            }
        }
    }

    fn is_open(&self, i: usize, j: usize) -> bool {
        if self.is_valid_index(i, j) {
            self.grid[i][j]
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn is_full(&self, i: usize, j: usize) -> bool {
        if !self.is_open(i, j) {
            return false;
        }

        if self.uf.connected(i * self.size + j, self.size * self.size) {
            return true;
        }
        false
    }    

    fn percolates(&self) -> bool {
        if self.uf.connected(self.size * self.size, self.size * self.size + 1) {
            return true;
        }
        false
    }

    fn is_valid_index(&self, i: usize, j: usize) -> bool {
        i < self.size && j < self.size
    }

    // fn dfs(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> bool {
    //     if i == 0 {
    //         return true;
    //     }

    //     visited[i][j] = true;
    //     let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
    //     for (di, dj) in directions.iter() {
    //         let ni = i as i32 + di;
    //         let nj = j as i32 + dj;
            
    //         if ni >= 0 && ni < self.size as i32 && nj >= 0 && nj < self.size as i32 {
    //             let ni = ni as usize;
    //             let nj = nj as usize;
    //             if !visited[ni][nj] && self.is_open(ni, nj) {
    //                 if self.dfs(ni, nj, visited) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }
}

struct PercolationStats {
    #[allow(dead_code)]
    n: usize,
    trials: usize,
    thresholds: Vec<f64>,
}

impl PercolationStats {
    fn new(n: usize, trials: usize) -> Self {
        let mut stats = PercolationStats {
            n,
            trials,
            thresholds: Vec::with_capacity(trials),
        };
        
        for _ in 0..trials {
            let mut perc = Percolation::new(n);
            let mut opened = 0;
            
            while !perc.percolates() {
                let mut i;
                let mut j;
                loop {
                    i = rand::rng().random_range(0..n);
                    j = rand::rng().random_range(0..n);
                    if !perc.is_open(i, j) {
                        break;
                    }
                }
                perc.open(i, j);
                opened += 1;
            }
            
            stats.thresholds.push(opened as f64 / (n * n) as f64);
        }
        
        stats
    }

    fn mean(&self) -> f64 {
        let sum: f64 = self.thresholds.iter().sum();
        sum / self.trials as f64
    }

    fn stddev(&self) -> f64 {
        if self.trials == 1 {
            return 0.0;
        }
        
        let mean = self.mean();
        let variance: f64 = self.thresholds.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (self.trials - 1) as f64;
        variance.sqrt()
    }

    fn confidence_lo(&self) -> f64 {
        let mean = self.mean();
        let stddev = self.stddev();
        mean - 1.96 * stddev / (self.trials as f64).sqrt()
    }

    fn confidence_hi(&self) -> f64 {
        let mean = self.mean();
        let stddev = self.stddev();
        mean + 1.96 * stddev / (self.trials as f64).sqrt()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("IllegalArgumentException");
        std::process::exit(1);
    }

    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("IllegalArgumentException");
            std::process::exit(1);
        }
    };

    let trials: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("IllegalArgumentException"); 
            std::process::exit(1);
        }
    };

    let stats = PercolationStats::new(n, trials);
    println!("均值 = {:.6}", stats.mean());
    println!("标准差 = {:.6}", stats.stddev());
    println!("95% 置信区间 = [{:.6}, {:.6}]", stats.confidence_lo(), stats.confidence_hi());
}
