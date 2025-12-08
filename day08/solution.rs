use std::path::Path;

#[path = "../utils.rs"]
mod utils;

struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn distance(&self, other: &Point3D) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn parse_coordinates(lines: &[String]) -> Vec<Point3D> {
    lines.iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            Point3D { x: parts[0], y: parts[1], z: parts[2] }
        })
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false;
        }
        
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }
    
    fn count_circuits(&mut self, n: usize) -> Vec<usize> {
        let mut circuit_sizes = Vec::new();
        let mut visited = vec![false; n];
        
        for i in 0..n {
            let root = self.find(i);
            if !visited[root] {
                visited[root] = true;
                circuit_sizes.push(self.size[root]);
            }
        }
        circuit_sizes
    }
}

fn part1(lines: &[String], num_connections: usize) -> i64 {
    let points = parse_coordinates(lines);
    let n = points.len();
    
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j, points[i].distance(&points[j])));
        }
    }
    
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    let mut uf = UnionFind::new(n);
    let mut attempts = 0;
    
    for (i, j, _) in edges.iter() {
        attempts += 1;
        
        if uf.find(*i) != uf.find(*j) {
            uf.union(*i, *j);
        }
        
        if attempts >= num_connections {
            break;
        }
    }
    
    let mut circuit_sizes = uf.count_circuits(n);
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    
    circuit_sizes.iter().take(3).map(|&s| s as i64).product()
}

fn part2(lines: &[String]) -> i64 {
    let points = parse_coordinates(lines);
    let n = points.len();
    
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j, points[i].distance(&points[j])));
        }
    }
    
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    
    let mut uf = UnionFind::new(n);
    
    for (i, j, _) in edges.iter() {
        if uf.find(*i) == uf.find(*j) {
            continue;
        }
        
        uf.union(*i, *j);
        
        if uf.count_circuits(n).len() == 1 {
            return (points[*i].x * points[*j].x) as i64;
        }
    }
    
    0
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let lines = utils::read_lines(&input_path);

    println!("Part 1: {}", part1(&lines, 1000));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        assert_eq!(part1(&input, 10), 40);
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        assert_eq!(part2(&input), 25272);
    }
}
