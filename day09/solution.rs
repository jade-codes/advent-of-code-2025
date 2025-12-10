use std::path::Path;

// Add the parent directory to access utils
#[path = "../utils.rs"]
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_points(lines: &[String]) -> Vec<Point> {
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

fn is_point_inside_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let pi = polygon[i];
        let pj = polygon[j];
        
        if ((pi.y > point.y) != (pj.y > point.y)) &&
           (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
    }
    
    inside
}

// Check if a rectangle is completely inside the polygon
fn is_rectangle_inside_polygon(p1: Point, p2: Point, polygon: &[Point]) -> bool {
    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);
    
    // Check all four corners are inside or on the polygon boundary
    let corners = [
        Point { x: min_x, y: min_y },
        Point { x: min_x, y: max_y },
        Point { x: max_x, y: min_y },
        Point { x: max_x, y: max_y },
    ];
    
    for corner in &corners {
        if !is_point_inside_polygon(*corner, polygon) && !polygon.contains(corner) {
            return false;
        }
    }
    
    // Check that rectangle edges don't cross polygon edges
    let rect_edges = [
        (Point { x: min_x, y: min_y }, Point { x: max_x, y: min_y }), // bottom
        (Point { x: max_x, y: min_y }, Point { x: max_x, y: max_y }), // right
        (Point { x: max_x, y: max_y }, Point { x: min_x, y: max_y }), // top
        (Point { x: min_x, y: max_y }, Point { x: min_x, y: min_y }), // left
    ];
    
    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let poly_edge = (polygon[i], polygon[j]);
        
        for rect_edge in &rect_edges {
            if edges_intersect(rect_edge.0, rect_edge.1, poly_edge.0, poly_edge.1) {
                return false;
            }
        }
    }
    
    true
}

fn edges_intersect(a1: Point, a2: Point, b1: Point, b2: Point) -> bool {
    if a1 == b1 || a1 == b2 || a2 == b1 || a2 == b2 {
        return false;
    }
    
    fn ccw(a: Point, b: Point, c: Point) -> i64 {
        (c.y - a.y) * (b.x - a.x) - (b.y - a.y) * (c.x - a.x)
    }
    
    let d1 = ccw(b1, b2, a1);
    let d2 = ccw(b1, b2, a2);
    let d3 = ccw(a1, a2, b1);
    let d4 = ccw(a1, a2, b2);
    
    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) &&
    ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

fn calculate_rectangle_area(p1: Point, p2: Point) -> i64 {
    let width = (p1.x - p2.x).abs() + 1;
    let height = (p1.y - p2.y).abs() + 1;
    width * height
}

fn part1(lines: &[String]) -> i64 {
    let points = parse_points(lines);
    
    if points.len() < 2 {
        return 0;
    }
    
    let mut max_area = 0;
    
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let area = calculate_rectangle_area(points[i], points[j]);
            max_area = max_area.max(area);
        }
    }
    
    max_area
}

fn part2(lines: &[String]) -> i64 {
    let points = parse_points(lines);
    
    if points.len() < 2 {
        return 0;
    }

    let mut max_area = 0;
    
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if is_rectangle_inside_polygon(points[i], points[j], &points) {
                let area = calculate_rectangle_area(points[i], points[j]);
                max_area = max_area.max(area);
            }
        }
    }
    
    max_area
}

pub fn main() {
    let input_path = Path::new(file!()).parent().unwrap().join("input.txt");
    let lines = utils::read_lines(&input_path);

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_points() {
        let input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
        ];
        let points = parse_points(&input);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point { x: 7, y: 1 });
        assert_eq!(points[1], Point { x: 11, y: 1 });
        assert_eq!(points[2], Point { x: 11, y: 7 });
    }

    #[test]
    fn test_calculate_rectangle_area() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 5, y: 3 };
        assert_eq!(calculate_rectangle_area(p1, p2), 15); // (5-1+1) * (3-1+1) = 5 * 3
        
        let p3 = Point { x: 9, y: 5 };
        let p4 = Point { x: 2, y: 3 };
        assert_eq!(calculate_rectangle_area(p3, p4), 24); // (9-2+1) * (5-3+1) = 8 * 3
    }

    #[test]
    fn test_point_inside_polygon() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 10, y: 10 },
            Point { x: 0, y: 10 },
        ];
        
        // Point clearly inside
        assert!(is_point_inside_polygon(Point { x: 5, y: 5 }, &polygon));
        
        // Point clearly outside
        assert!(!is_point_inside_polygon(Point { x: 15, y: 15 }, &polygon));
        assert!(!is_point_inside_polygon(Point { x: -5, y: 5 }, &polygon));
    }

    #[test]
    fn test_edges_intersect() {
        let a1 = Point { x: 0, y: 0 };
        let a2 = Point { x: 10, y: 10 };
        let b1 = Point { x: 0, y: 10 };
        let b2 = Point { x: 10, y: 0 };
        assert!(edges_intersect(a1, a2, b1, b2));
        
        let c1 = Point { x: 0, y: 0 };
        let c2 = Point { x: 5, y: 0 };
        let d1 = Point { x: 0, y: 5 };
        let d2 = Point { x: 5, y: 5 };
        assert!(!edges_intersect(c1, c2, d1, d2));
        
        let e1 = Point { x: 0, y: 0 };
        let e2 = Point { x: 5, y: 5 };
        let f1 = Point { x: 5, y: 5 };
        let f2 = Point { x: 10, y: 0 };
        assert!(!edges_intersect(e1, e2, f1, f2));
    }

    #[test]
    fn test_part1_example() {
        let input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2_example() {
        let input = vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        assert_eq!(part2(&input), 24);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_single_point() {
        let input = vec!["5,5".to_string()];
        assert_eq!(part1(&input), 0);
        assert_eq!(part2(&input), 0);
    }
}
