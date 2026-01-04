use crate::grid::Point;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// Internal `State` used in the priority queue (min-heap behaviour).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    node: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering on cost so BinaryHeap acts like a min-heap.
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A simple directed weighted graph with `Point` nodes.
///
/// Nodes are created implicitly when an edge is added.
pub struct Graph {
    edges: HashMap<Point, Vec<(Point, usize)>>,
}

impl Graph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    /// Add a directed edge from `from` to `to` with the given non-negative `cost`.
    ///
    /// Nodes are implicitly created if they did not previously exist in the graph.
    pub fn add_edge(&mut self, from: Point, to: Point, cost: usize) {
        self.edges.entry(from).or_default().push((to, cost));
        self.edges.entry(to).or_default();
    }

    /// Returns the cost of the shortest path from `start` to `goal` using Dijkstra's algorithm.
    ///
    /// If no path exists, returns `None`.
    pub fn dijkstra(&self, start: Point, goal: Point) -> Option<usize> {
        if start == goal {
            return Some(0);
        }

        if !self.edges.contains_key(&start) || !self.edges.contains_key(&goal) {
            return None;
        }

        self.dijkstra_many(&[start], goal)
    }

    /// Returns the cost of the shortest path from any of the start position to the goal
    ///
    /// If no path exists, returns `None`.
    pub fn dijkstra_many(&self, start: &[Point], goal: Point) -> Option<usize> {
        let mut dist: HashMap<Point, usize> = HashMap::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        for &s in start {
            dist.insert(s, 0);
            heap.push(State { cost: 0, node: s });
        }

        while let Some(State { cost, node }) = heap.pop() {
            if cost > *dist.get(&node).unwrap_or(&usize::MAX) {
                // already found a shorter path to this node
                continue;
            }

            if node == goal {
                return Some(cost);
            }

            if let Some(neighbours) = self.edges.get(&node) {
                for (neighbour, weight) in neighbours.iter() {
                    let next_cost = cost.saturating_add(*weight);

                    if next_cost < *dist.get(neighbour).unwrap_or(&usize::MAX) {
                        dist.insert(*neighbour, next_cost);

                        heap.push(State {
                            cost: next_cost,
                            node: *neighbour,
                        });
                    }
                }
            }
        }

        None
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_shortest_path() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 0);
        let c = Point::new(2, 0);

        let mut g = Graph::new();
        g.add_edge(a, b, 5);
        g.add_edge(b, c, 2);
        g.add_edge(a, c, 10);

        assert_eq!(g.dijkstra(a, c), Some(7));
        assert_eq!(g.dijkstra(a, b), Some(5));
        assert_eq!(g.dijkstra(b, a), None); // directed edge only
    }

    #[test]
    fn disconnected_nodes() {
        let a = Point::new(0, 0);
        let b = Point::new(1, 1);
        let mut g = Graph::new();
        g.add_edge(a, a, 0); // self-loop to create node a
        // b exists but no edges in or out
        g.add_edge(b, b, 0);
        // remove any connecting edges
        assert_eq!(g.dijkstra(a, Point::new(10, 10)), None);
    }

    #[test]
    fn start_equals_goal_returns_zero() {
        let p = Point::new(3, 4);
        let mut g = Graph::new();
        // even if node not present, we consider start==goal -> 0
        assert_eq!(g.dijkstra(p, p), Some(0));

        // when nodes are present
        g.add_edge(p, Point::new(0, 0), 1);
        assert_eq!(g.dijkstra(p, p), Some(0));
    }
}
