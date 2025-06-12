use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

type Graph = HashMap<usize, Vec<(usize, usize)>>; // 邻接表表示：顶点 -> [(邻接顶点, 边权重)]

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    vertex: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // 最小堆
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DijkstraOptimizer {
    graph: Graph,
    prev_distances: HashMap<usize, usize>, // 上次查询的距离缓存
    touched: Vec<bool>,                   // 标记在上次查询中被修改的顶点
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

        // 部分重新初始化：只重置被修改过的距离
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
    let mut graph = Graph::new();
    graph.insert(0, vec![(1, 7), (2, 9), (5, 14)]);
    graph.insert(1, vec![(0, 7), (2, 10), (3, 15)]);
    graph.insert(2, vec![(0, 9), (1, 10), (3, 11), (5, 2)]);
    graph.insert(3, vec![(1, 15), (2, 11), (4, 6)]);
    graph.insert(4, vec![(3, 6), (5, 9)]);
    graph.insert(5, vec![(0, 14), (2, 2), (4, 9)]);

    let mut optimizer = DijkstraOptimizer::new(graph);

    println!("0 -> 4: {:?}", optimizer.query(0, 4)); // 应该是 20
    println!("1 -> 4: {:?}", optimizer.query(1, 4)); // 应该是 21
    println!("2 -> 5: {:?}", optimizer.query(2, 5)); // 应该是 2
}
