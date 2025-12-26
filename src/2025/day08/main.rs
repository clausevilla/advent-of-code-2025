use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn distance_sq(a: &Point, b: &Point) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // RETURN whether a merge actually happened
    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }

        self.components -= 1;
        true
    }
}

fn main() {
    let contents = fs::read_to_string("boxes.txt")
        .expect("Failed to read file");

    let points: Vec<Point> = contents
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    let n = points.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..n {             // generate all pairwise distances
        for j in (i + 1)..n {
            let d = distance_sq(&points[i], &points[j]);
            edges.push((d, i, j));
        }
    }

    // sort by distance
    edges.sort_unstable_by_key(|e| e.0);

    // Part 1:
    let mut uf_part1 = UnionFind::new(n);

    // Process first 1000 shortest connections
    for &(_, a, b) in edges.iter().take(1000) {
        uf_part1.union(a, b);
    }

    let mut component_sizes = vec![0usize; n];  // count component sizes
    for i in 0..n {
        let root = uf_part1.find(i);
        component_sizes[root] += 1;
    }

    // Get the 3 largest
    component_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let part1 = component_sizes[0] * component_sizes[1] * component_sizes[2];

    println!("Part 1 output: {}", part1);

    // Part 2:
    let mut uf_part2 = UnionFind::new(n);
    let mut last_edge = None;

    for &(_, a, b) in &edges {
        if uf_part2.union(a, b) {
            last_edge = Some((a, b));

            if uf_part2.components == 1 {
                break;
            }
        }
    }

    let (a, b) = last_edge.expect("Never fully connected");
    let part2 = points[a].x * points[b].x;

    println!("Part 2 output: {}", part2);
}