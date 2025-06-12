use std::collections::{BinaryHeap, HashMap};
use std::f64;
use std::cmp::Ordering;

type Point = (f64, f64);
type Graph = HashMap<usize, (Point, Vec<(usize, f64)>)>; 

fn euclidean_distance(a: &Point, b: &Point) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

#[derive(Copy, Clone, PartialEq)]
struct State {
    f_score: f64, 
    vertex: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct AStarSearcher {
    graph: Graph,
    g_scores: HashMap<usize, f64>,  
    came_from: HashMap<usize, usize>,
}

impl AStarSearcher {
    fn new(graph: Graph) -> Self {
        AStarSearcher {
            graph,
            g_scores: HashMap::new(),
            came_from: HashMap::new(),
        }
    }

    fn search(&mut self, start: usize, goal: usize) -> Option<(Vec<usize>, f64)> {
        self.g_scores.clear();
        self.came_from.clear();

        let goal_point = self.graph.get(&goal)?.0;
        let mut open_set = BinaryHeap::new();
        let mut in_open_set = vec![false; self.graph.len()];

        // 初始化起始点
        self.g_scores.insert(start, 0.0);
        open_set.push(State {
            f_score: euclidean_distance(&self.graph.get(&start)?.0, &goal_point),
            vertex: start,
        });
        in_open_set[start] = true;

        while let Some(State { vertex: current, .. }) = open_set.pop() {
            if current == goal {
                return Some((self.reconstruct_path(goal), *self.g_scores.get(&goal)?));
            }

            in_open_set[current] = false;

            let (current_point, neighbors) = self.graph.get(&current)?;
            for &(neighbor, edge_dist) in neighbors {
                let tentative_g = self.g_scores[&current] + edge_dist;

                if tentative_g < *self.g_scores.get(&neighbor).unwrap_or(&f64::INFINITY) {
                    self.came_from.insert(neighbor, current);
                    self.g_scores.insert(neighbor, tentative_g);

                    let neighbor_point = self.graph.get(&neighbor)?.0;
                    let h = euclidean_distance(&neighbor_point, &goal_point); 
                    let f = tentative_g + h;

                    if !in_open_set[neighbor] {
                        open_set.push(State {
                            f_score: f,
                            vertex: neighbor,
                        });
                        in_open_set[neighbor] = true;
                    }
                }
            }
        }

        None
    }

    fn reconstruct_path(&self, goal: usize) -> Vec<usize> {
        let mut path = vec![goal];
        let mut current = goal;

        while let Some(&prev) = self.came_from.get(&current) {
            path.push(prev);
            current = prev;
        }

        path.reverse();
        path
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.insert(0, ((0.0, 0.0), vec![]));
    graph.insert(1, ((2.0, 2.0), vec![]));
    graph.insert(2, ((2.0, -2.0), vec![]));
    graph.insert(3, ((5.0, 0.0), vec![]));
    graph.insert(4, ((7.0, 2.0), vec![]));
    graph.insert(5, ((7.0, -2.0), vec![]));

    let edges = vec![
        (0, 1), (1, 0),
        (0, 2), (2, 0),
        (1, 2), (2, 1),
        (1, 3), (3, 1),
        (2, 3), (3, 2),
        (3, 4), (4, 3),
        (3, 5), (5, 3),
        (4, 5), (5, 4),
        (1, 4), (2, 5)
    ];

    for (u, v) in edges {
        let pos_u = graph.get(&u).unwrap().0;
        let pos_v = graph.get(&v).unwrap().0;
        let dist = euclidean_distance(&pos_u, &pos_v);
        graph.get_mut(&u).unwrap().1.push((v, dist));
    }

    let mut searcher = AStarSearcher::new(graph);

    if let Some((path, cost)) = searcher.search(0, 5) {
        println!("找到路径: {:?}", path);
        println!("总成本: {:.3}", cost);
    } else {
        println!("未找到路径");
    }
}
