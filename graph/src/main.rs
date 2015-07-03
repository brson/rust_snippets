use std::iter::repeat;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn max(a: u32, b: u32) -> u32 { if a > b { a } else { b } }

#[derive(Clone, PartialEq, Eq)]
struct Edge {
    vertex: u32,
    weight: f32,
}

impl Edge {
    pub fn new(vertex_: u32, weight_: f32) -> Edge {
        Edge{vertex: vertex_, weight: weight_,}
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.weight, &self.vertex).cmp(&(other.weight, &other.vertex))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    nodes: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Graph { Graph{nodes: Vec::new(),} }

    pub fn add_edge(&mut self, src: u32, dst: u32, weight: f32) {
        let len = self.nodes.len();

        if (max(src, dst)) > len as u32 {
            let new_len = (max(src, dst) + 1) as usize;
            self.nodes.extend(repeat(Vec::new()).take(new_len - len))
        }
        self.nodes[src as usize].push(Edge::new(dst, weight));
    }
    pub fn bfs(&self, src: u32) {
        let mut queue: VecDeque<u32> = VecDeque::new();
        let len = self.nodes.len();
        let mut visited = vec!(false ; len);
        
        queue.push_front(src);

        while let Some(current) = queue.pop_back() {
            if !visited[current as usize] {
                println!("current: {}" , current);
                visited[current as usize] = true;
            } else { continue ; }

            for n in &self.nodes[current as usize] {
                let neighbor: u32 = n.vertex;
                queue.push_front(neighbor);
            }
        }
    }
    pub fn dfs(&self, src: u32) {
        let mut stack: Vec<u32> = Vec::new();
        let len = self.nodes.len() as usize;
        let mut visited = vec!(false ; len);

        stack.push(src);
        
        while let Some(current) = stack.pop() {
            if !visited[current as usize] {
                println!("current: {}" , current);
                visited[current as usize] = true;
            } else { continue ; }

            for n in &self.nodes[current as usize] {
                let neighbor: u32 = n.vertex;
                stack.push(neighbor);
            }
        }
    }

    pub fn dijkstra(&self, src: u32, dst: u32) {
      let mut dist: Vec<f32> = Vec::new();
      let mut prev: Vec<u32> = Vec::new();
      let mut q = BinaryHeap::new();
      const max_weight: f32 = std::f32::MAX;

      // init dist, prev table
      for i in 0..self.nodes.len() {
        dist.push(max_weight);
        prev.push(0);
      }

      // We're at `start`, with a zero cost
      dist[src as usize] = 0.0;
      q.push(Edge::new(src, dist[src as usize]));

      //while (!q.is_empty()) {
      while let Some(u) = q.pop() {
        //let u: Edge = el.unwrap();

        // loop for all edges connected to
        for v in self.nodes[u.vertex as usize].iter() {

          let alt: f32 = dist[u.vertex as usize] + v.weight; // accumulate shortest dist from source

          if (alt < dist[v.vertex as usize]) {
            dist[v.vertex as usize] = alt; // keep the shortest dist from src to v
            prev[v.vertex as usize] = u.vertex;

            q.push(Edge::new(v.vertex, dist[v.vertex as usize])); // Add unvisited v into the Q to be processed  
          }
        }
      }

      let mut shortestPath: Vec<u32> = Vec::new();
      let mut curr: u32 = dst;

      shortestPath.push(curr);
    
      while (curr != src) {
        curr = prev[curr as usize];
        shortestPath.push(curr);
      }
    
      shortestPath.reverse();

      for v in shortestPath.iter() {
        println!("{}", v);
      }
    }
}

fn main() {
    let mut g1 = Graph::new();
    g1.add_edge(0, 1, 1.0);
    g1.add_edge(0, 2, 4.0);
    g1.add_edge(1, 0, 1.0);
    g1.add_edge(1, 2, 2.0);
    g1.add_edge(1, 3, 6.0);
    g1.add_edge(2, 0, 4.0);
    g1.add_edge(2, 1, 2.0);
    g1.add_edge(2, 3, 3.0);
    g1.add_edge(3, 1, 6.0);
    g1.add_edge(3, 2, 3.0);
    
    g1.bfs(3);
    
    println!("\n");
    
    g1.dfs(3);

    g1.dijkstra(0, 2);
}