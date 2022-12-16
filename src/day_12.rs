use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct Edge {
    to: (usize, usize),
    cost: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| 
                (self.position.0 + self.position.1).cmp(&(other.position.0 + other.position.1))
            )
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Vec<Edge>>>, start: (usize, usize), goal: (usize, usize)) -> Option<(i32, Vec<Vec<i32>>)> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<Vec<i32>> = (0..adj_list.len()).map(|_| vec!()).collect();

    for row in dist.iter_mut() {
        let mut x = (0..adj_list[0].len()).map(|_| i32::MAX).collect();
        row.append(&mut x);
    }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.1][start.0] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {

        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some((cost, dist)); }

        // Important as we may have already found a better way
        if cost > dist[position.1][position.0] { 
            continue; 
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position.1][position.0] {
            let next = State { cost: cost + edge.cost, position: edge.to };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position.1][next.position.0] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position.1][next.position.0] = next.cost;
            }
        }

        if heap.is_empty() {
            println!("Last position: {:?} {}", position, cost);
        }
    }

    // Goal not reachable
    Some((0, dist))
}

fn main() {
    let input = std::fs::read_to_string("day_12 input.txt").unwrap();

    let width = input.lines().next().expect("At least one line").len();
    let height = input.lines().count();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut heights: Vec<Vec<i32>> = vec!();

    for (y, line) in input.lines().enumerate() {
        let mut row = vec!();
        
        for (x, value) in line.bytes().enumerate() {
            if value == 83 {    //S
                start = (x, y);
                row.push(0);
            }
            else if value == 69 {     //E
                end = (x, y);
                row.push(25);
            } else {
                row.push(value as i32 - 97);
            }
        }

        heights.push(row);
    }

    let dirs = [
        (0, -1),    //Up
        (1, 0),     //Right
        (-1, 0),    //Left
        (0, 1),    //Down
    ];

    let mut edge_list: Vec<Vec<Vec<Edge>>> = vec!();

    for y in 0..heights.len() {
        let mut row = vec!();

        for x in 0..heights[y].len() {
            let mut edges = vec!();
            let this_cost = heights[y][x];

            for dir in dirs {
                let other_x = x as i32 + dir.0;
                let other_y = y as i32 + dir.1;

                if other_x < 0 || other_y < 0 || other_x == width as i32 || other_y == height as i32 {
                    continue;
                } 
                
                let other_x = other_x as usize;
                let other_y = other_y as usize;

                let diff = heights[other_y][other_x] - this_cost;
                
                if diff.abs() <= 1 {
                    //only interested in edges we can traverse
                    edges.push(Edge { to: (other_x, other_y), cost: 1 });
                }
            }

            row.push(edges);
        }
        edge_list.push(row);
    }

    /*
    for (y, r) in heights.iter().enumerate() {
        for (x, h) in r.iter().enumerate() {
            if edge_list[y][x].is_empty() {
                print!("   ");
            } else {
                print!("{:>2} ", h);
            }
        }
        println!();
    }
    */

    if let Some((total, costs)) = shortest_path(&edge_list, start, end) {
        for row in costs {
            for col in row {
                if col == i32::MAX {
                    print!("  X ");
                } else {
                    print!("{:>3} ", col);
                }
            }
            println!();
        }

        println!("Cost: {:?}", total);
    }
}