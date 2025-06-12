use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

type Graph = HashMap<usize, Vec<(usize, usize)>>; 

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    vertex: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) 
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DijkstraOptimizer {
    graph: Graph,
    prev_distances: HashMap<usize, usize>,
    touched: Vec<bool>,                
}

impl DijkstraOptimizer {
    fn new(graph: Graph) -> Self {
        let size = graph.keys().max().map_or(0, |&x| x) + 1;
        DijkstraOptimizer {
            graph,
            prev_distances: HashMap::new(),
            touched: vec![false; size],
        }
    }

    fn query(&mut self, start: usize, end: usize) -> Option<usize> {
        let mut distances = self.prev_distances.clone();
        let mut heap = BinaryHeap::new();
        let mut visited = vec![false; self.touched.len()];

        for v in 0..self.touched.len() {
            if self.touched[v] {
                distances.insert(v, usize::MAX);
                self.touched[v] = false;
            }
        }

        distances.insert(start, 0);
        heap.push(State {
            cost: 0,
            vertex: start,
        });

        while let Some(State { cost, vertex }) = heap.pop() {
            if vertex == end {
                self.prev_distances = distances;
                return Some(cost);
            }

            if visited[vertex] {
                continue;
            }
            visited[vertex] = true;

            if let Some(edges) = self.graph.get(&vertex) {
                for &(neighbor, weight) in edges {
                    let new_cost = cost + weight;
                    let current_dist = *distances.get(&neighbor).unwrap_or(&usize::MAX);

                    if new_cost < current_dist {
                        distances.insert(neighbor, new_cost);
                        self.touched[neighbor] = true;
                        heap.push(State {
                            cost: new_cost,
                            vertex: neighbor,
                        });
                    }
                }
            }
        }

        self.prev_distances = distances;
        None
    }
}

fn main() {
    // 示例图
    let mut graph = Graph::new();
    graph.insert(0, vec![(1, 4), (2, 2)]);
    graph.insert(1, vec![(3, 5)]);
    graph.insert(2, vec![(1, 1), (3, 8)]);
    graph.insert(3, vec![]);

    let mut optimizer = DijkstraOptimizer::new(graph);

    println!("0 -> 3: {:?}", optimizer.query(0, 3)); 
    println!("1 -> 3: {:?}", optimizer.query(1, 3)); 
    println!("2 -> 3: {:?}", optimizer.query(2, 3)); 
}