use std::collections::{VecDeque, HashMap};
use std::env;
use std::fs;

struct Node {
    children: HashMap<char, usize>,
    fail: usize,
    outputs: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            fail: 0,
            outputs: Vec::new(),
        }
    }
}

struct ACAutomaton {
    nodes: Vec<Node>,
}

impl ACAutomaton {
    fn new() -> Self {
        let mut nodes = Vec::new();
        nodes.push(Node::new()); // root
        ACAutomaton { nodes }
    }

    fn insert(&mut self, pattern: &str, index: usize) {
        let mut current = 0;
        for c in pattern.chars() {
            if !self.nodes[current].children.contains_key(&c) {
                self.nodes.push(Node::new());
                let new_index = self.nodes.len() - 1;
                self.nodes[current].children.insert(c, new_index);
            }
            current = self.nodes[current].children[&c];
        }
        self.nodes[current].outputs.push(index);
    }

    fn build_failures(&mut self) {
        let mut queue = VecDeque::new();

        let children: Vec<(char, usize)> = self.nodes[0].children.iter().map(|(&c, &i)| (c, i)).collect();
        for (_c, child) in children {
            self.nodes[child].fail = 0;
            queue.push_back(child);
        }

        while let Some(current) = queue.pop_front() {
            let child_list: Vec<(char, usize)> = self.nodes[current].children.iter().map(|(&c, &i)| (c, i)).collect();

            for (c, child) in child_list {
                let mut fail = self.nodes[current].fail;

                while fail != 0 && !self.nodes[fail].children.contains_key(&c) {
                    fail = self.nodes[fail].fail;
                }

                let next_fail = if let Some(&f) = self.nodes[fail].children.get(&c) {
                    f
                } else {
                    0
                };

                self.nodes[child].fail = next_fail;

                let outputs = self.nodes[next_fail].outputs.clone();
                self.nodes[child].outputs.extend(outputs);

                queue.push_back(child);
            }
        }
    }

    fn search_char(&self, text: &str, patterns: &[String]) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        let mut state = 0;
        let chars: Vec<char> = text.chars().collect();

        for i in 0..chars.len() {
            let c = chars[i];

            while state != 0 && !self.nodes[state].children.contains_key(&c) {
                state = self.nodes[state].fail;
            }

            if let Some(&next_state) = self.nodes[state].children.get(&c) {
                state = next_state;
            }

            for &pattern_index in &self.nodes[state].outputs {
                result.entry(pattern_index).or_insert(i + 1 - patterns[pattern_index].len());
            }
        }

        result
    }

    fn search_character(&self, text: &str, patterns: &[String]) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let mut state = 0;
    let chars: Vec<char> = text.chars().collect();

    let mut word_indices = vec![0; chars.len()];
    let mut word_count = 0;
    let mut in_word = false;

    for i in 0..chars.len() {
        if chars[i].is_ascii_whitespace() {
            in_word = false;
        } else {
            if !in_word {
                word_count += 1;
                in_word = true;
            }
        }
        word_indices[i] = word_count ;
    }

    for i in 0..chars.len() {
        let c = chars[i];

        while state != 0 && !self.nodes[state].children.contains_key(&c) {
            state = self.nodes[state].fail;
        }

        if let Some(&next_state) = self.nodes[state].children.get(&c) {
            state = next_state;
        }

        for &pattern_index in &self.nodes[state].outputs {
            let pat_word_len = patterns[pattern_index].split_whitespace().count();
            let current_word_pos = word_indices[i];
            if current_word_pos + 1 >= pat_word_len {
                let start_word_index = current_word_pos + 1 - pat_word_len;
                result.entry(pattern_index).or_insert(start_word_index);
            }
        }
    }

    result
}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <corpus_file> <query_file>", args[0]);
        std::process::exit(1);
    }

    let corpus = fs::read_to_string(&args[1]).expect("Failed to read corpus file");
    let patterns: Vec<String> = fs::read_to_string(&args[2])
        .expect("Failed to read query file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut ac = ACAutomaton::new();
    for (i, pattern) in patterns.iter().enumerate() {
        ac.insert(pattern, i);
    }
    ac.build_failures();

    let positions = ac.search_character(&corpus, &patterns);

    for (i, pattern) in patterns.iter().enumerate() {
        if let Some(&pos) = positions.get(&i) {
            println!("{} {}", pos, pattern);
        } else {
            println!("-- {}", pattern);
        }
    }
}
