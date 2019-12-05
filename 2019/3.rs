use std::io;
use std::io::prelude::*;

#[derive(Clone,Copy,PartialEq,Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn get_orientation(&self) -> Orientation {
        if self.start.y == self.end.y {
            return Orientation::Horizontal;
        } else {
            return Orientation::Vertical;
        }
    }

    fn contains_point(&self, point:&Point) -> bool {
        match self.get_orientation() {
            Orientation::Horizontal => {
                return self.start.y == point.y && ((self.start.x < point.x && point.x < self.end.x ) || (self.end.x < point.x && point.x < self.start.x));
            },
            Orientation::Vertical => {
                return self.start.x == point.x && ((self.start.y < point.y && point.y < self.end.y ) || (self.end.y < point.y && point.y < self.start.y));
            },
        }
    }
}

#[derive(Clone,Copy,PartialEq,Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(PartialEq,Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq,Debug)]
struct Vector {
    direction: Direction,
    length: i32,
}

fn get_path_from_string(string:&str) -> Vec<Vector> {
    let mut path:Vec<Vector> = vec![];

    for token in string.split(","){
        let mut token = token.to_string();

        let dir = match token.remove(0) {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => Direction::None,
        };

        let v = Vector{
            direction: dir,
            length: token.parse::<i32>().unwrap(),
        };

        path.push(v);
     }

    return path;
}

fn get_segments_from_path(start:Point, path:Vec<Vector>) -> Vec<Segment> {
    let mut current = start;
    let mut segments = vec![];

    for vector in path {
        let end = match vector.direction {
            Direction::Up => Point { x: current.x, y: current.y + vector.length },
            Direction::Down => Point { x: current.x, y: current.y - vector.length },
            Direction::Left => Point { x: current.x - vector.length, y: current.y },
            Direction::Right => Point { x: current.x + vector.length, y: current.y },
            Direction::None => current,
        };

        let segment = Segment {
            start: current,
            end: end,
        };

        segments.push(segment);

        current = end.clone();
    }

    return segments;
}

fn find_intersections_of_lines(path1:Vec<Segment>, path2:Vec<Segment>) -> Vec<Point> {
    let mut intersections = vec![];

    for segment1 in &path1 {
        for segment2 in &path2 {
            if segment1.get_orientation() == segment2.get_orientation() {
                continue;
            }

            match segment1.get_orientation() {
                Orientation::Horizontal => {
                    if ((segment1.start.x < segment2.start.x && segment2.start.x < segment1.end.x) ||
                       (segment1.end.x < segment2.start.x && segment2.start.x < segment1.start.x )) &&
                       ((segment2.end.y < segment1.start.y && segment1.start.y < segment2.start.y) ||
                       (segment2.start.y < segment1.start.y && segment1.start.y < segment2.end.y) ) {
                        intersections.push(Point{ x: segment2.start.x, y: segment1.start.y });
                        // println!("added intersection");
                        // println!("point: {:?}", Point{ x: segment2.start.x, y: segment1.start.y });
                        // println!("segment 1: {:?}", segment1);
                        // println!("segment 2: {:?}", segment2);
                        // println!("segment 1 orientation: {:?}", segment1.get_orientation());
                        // println!("segment 2 orientation: {:?}", segment2.get_orientation());
                    }
                },
                Orientation::Vertical => {
                    if ((segment1.start.y < segment2.start.y && segment2.start.y < segment1.end.y) ||
                       (segment1.end.y < segment2.start.y && segment2.start.y < segment1.start.y)) &&
                       ((segment2.start.x < segment1.start.x && segment1.start.x < segment2.end.x) ||
                       (segment2.end.x < segment1.start.x && segment1.start.x < segment2.start.x)) {
                        intersections.push(Point{ x: segment1.start.x, y: segment2.start.y });
                        // println!("added intersection");
                        // println!("point: {:?}", Point{ x: segment1.start.x, y: segment2.start.y });
                        // println!("segment 1: {:?}", segment1);
                        // println!("segment 2: {:?}", segment2);
                        // println!("segment 1 orientation: {:?}", segment1.get_orientation());
                        // println!("segment 2 orientation: {:?}", segment2.get_orientation());
                    }
                },
            }
        }
    }

    return intersections;
}

fn calculate_manhattan_distance(point1:Point, point2:Point) -> i32 {
    return (point1.x - point2.x).abs() + (point1.y - point2.y).abs();
}

fn find_closest_intersection(lines:Vec<String>) -> (i32, Point) {
    let wire_1_path = get_path_from_string(&lines[0]);
    let wire_2_path = get_path_from_string(&lines[1]);

    let wire_1_segments = get_segments_from_path(Point{x:1, y:1}, wire_1_path);
    let wire_2_segments = get_segments_from_path(Point{x:1, y:1}, wire_2_path);

    let intersections = find_intersections_of_lines(wire_1_segments, wire_2_segments);

    let mut shortest_distance = 1000;
    let mut closest_intersection = Point{x:1, y:1};

    for intersection in intersections {
        let dist = calculate_manhattan_distance(Point{x:1, y:1}, intersection);

        if dist < shortest_distance {
            shortest_distance = dist;
            closest_intersection = intersection;
        }
    }

    return (shortest_distance, closest_intersection);
}

fn find_shortest_path_to_intersection(lines:Vec<String>) -> (i32, Point) {
    let wire_1_path = get_path_from_string(&lines[0]);
    let wire_2_path = get_path_from_string(&lines[1]);

    let wire_1_segments = get_segments_from_path(Point{x:1, y:1}, wire_1_path);
    let wire_2_segments = get_segments_from_path(Point{x:1, y:1}, wire_2_path);

    let intersections = find_intersections_of_lines(wire_1_segments.clone(), wire_2_segments.clone());

    let mut shortest_path = 100000;
    let mut shortest_intersection = Point{x:1, y:1};

    for intersection in intersections {
        let get_path_length = | segments:Vec<Segment> | {
            let mut path_length = 0;

            for segment in segments {
                if segment.contains_point(&intersection) {
                    match segment.get_orientation() {
                        Orientation::Horizontal => {
                            path_length += (intersection.x - segment.start.x).abs();
                        },
                        Orientation::Vertical => {
                            path_length += (intersection.y - segment.start.y).abs();
                        },
                    }

                    println!("path_length: {}", path_length);

                    break;
                } else {
                    match segment.get_orientation() {
                        Orientation::Horizontal => {
                            path_length += (segment.end.x - segment.start.x).abs();
                        },
                        Orientation::Vertical => {
                            path_length += (segment.end.y - segment.start.y).abs();
                        },
                    }

                    println!("path_length: {}", path_length);
                }
            }

            return path_length;
        };

        let path_length_1 = get_path_length(wire_1_segments.clone());
        let path_length_2 = get_path_length(wire_2_segments.clone());
        let path_length = path_length_1 + path_length_2;

        if path_length < shortest_path {
            shortest_path = path_length;
            shortest_intersection = intersection;
        }
    }

    return (shortest_path, shortest_intersection);
}

fn main() -> io::Result<()> {
    let mut lines = vec![];

    let s = io::stdin();

    for line in s.lock().lines() {
        let l = line.unwrap().clone();

        if l == "" {
            break;
        }

        lines.push(l);
    }

    let (shortest_distance, closest_intersection) = find_closest_intersection(lines.clone());
    let (shortest_path, intersection) = find_shortest_path_to_intersection(lines.clone());

    println!("shortest_distance: {}", shortest_distance);
    println!("closest_intersection: {:?}", closest_intersection);

    println!("shortest_path: {}", shortest_path);
    println!("intersection: {:?}", intersection);

    Ok(())
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_get_path_from_string() {
        let path = get_path_from_string("U13,L4,R22,D1");

        assert_eq!(path.len(), 4);
        assert_eq!(path[0], Vector{direction:Direction::Up, length:13});
        assert_eq!(path[1], Vector{direction:Direction::Left, length:4});
        assert_eq!(path[2], Vector{direction:Direction::Right, length:22});
        assert_eq!(path[3], Vector{direction:Direction::Down, length:1});
    }

    #[test]
    fn test_get_segments_from_path() {
        let path = vec![Vector{direction:Direction::Up,length:13},
                        Vector{direction:Direction::Right, length:4},
                        Vector{direction:Direction::Left, length:2},
                        Vector{direction:Direction::Down, length:1}];

        let segments = get_segments_from_path(Point{ x: 1, y: 1 }, path);

        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0], Segment{ start: Point{ x: 1, y: 1 }, end: Point{ x: 1, y: 14} });
        assert_eq!(segments[1], Segment{ start: Point{ x: 1, y: 14 }, end: Point{ x: 5, y: 14} });
        assert_eq!(segments[2], Segment{ start: Point{ x: 5, y: 14 }, end: Point{ x: 3, y: 14} });
        assert_eq!(segments[3], Segment{ start: Point{ x: 3, y: 14 }, end: Point{ x: 3, y: 13} });
    }

    #[test]
    fn test_get_orientation_of_segment() {
        let seg1 = Segment{ start: Point{x:1, y:8}, end: Point{x:7, y:8} };
        let seg2 = Segment{ start: Point{x:7, y:8}, end: Point{x:7, y:4} };

        let orientation1 = seg1.get_orientation();
        let orientation2 = seg2.get_orientation();

        assert_eq!(orientation1, Orientation::Horizontal);
        assert_eq!(orientation2, Orientation::Vertical);
    }

    #[test]
    fn test_find_intersections_of_lines() {
        let line1 = vec![Segment{ start: Point{x:1, y:8}, end: Point{x:7, y:8} },
                         Segment{ start: Point{x:7, y:8}, end: Point{x:7, y:4} }];
        let line2 = vec![Segment{ start: Point{x:4, y:3}, end: Point{x:4, y:6} },
                         Segment{ start: Point{x:4, y:6}, end: Point{x:9, y:6} }];

        let intersections = find_intersections_of_lines(line1, line2);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0], Point{x:7, y:6});
    }

    #[test]
    fn test_calculate_manhattan_distance() {
        let dist = calculate_manhattan_distance(Point{x:1, y:1}, Point{x:4 , y:4});

        assert_eq!(dist, 6);
    }

    #[test]
    fn test_find_closest_intersection() {
        let lines = vec![String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")];

        let (shortest_distance, closest_intersection) = find_closest_intersection(lines);

        assert_eq!(shortest_distance, 6);
        assert_eq!(closest_intersection, Point{x:4, y:4});

        let lines = vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];

        let (shortest_distance, _) = find_closest_intersection(lines);

        assert_eq!(shortest_distance, 159);

        let lines = vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];

        let (shortest_distance, _) = find_closest_intersection(lines);

        assert_eq!(shortest_distance, 135);
    }

    #[test]
    fn test_find_shortest_path_to_intersection() {
        let lines = vec![String::from("R8,U5,L5,D3"), String::from("U7,R6,D4,L4")];

        let (shortest_path, intersection) = find_shortest_path_to_intersection(lines);

        assert_eq!(shortest_path, 30);
        assert_eq!(intersection, Point{x:7, y:6});

        let lines = vec![String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"), String::from("U62,R66,U55,R34,D71,R55,D58,R83")];

        let (shortest_path, _) = find_shortest_path_to_intersection(lines);

        assert_eq!(shortest_path, 610);

        let lines = vec![String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"), String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")];

        let (shortest_path, _) = find_shortest_path_to_intersection(lines);

        assert_eq!(shortest_path, 410);
    }
}
