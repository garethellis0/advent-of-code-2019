use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::{max, min};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Intersection {
    cell: Cell,
    path1: Path,
    path2: Path,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Orientation {
    UpDown,
    LeftRight,
}

/// A line segment between two points
#[derive(Clone, Eq, PartialEq, Debug)]
struct PathSegment {
    start_point: Cell,
    orientation: Orientation,
    length: i32,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Path {
    segments: Vec<PathSegment>,
}

fn read_lines(f: File) -> Vec<String> {
    let buffer = BufReader::new(f);

    let mut lines = Vec::new();
    for line in buffer.lines() {
        lines.push(line.unwrap())
    }

    return lines;
}

fn convert_str_to_path(str: &String) -> Result<Path, String> {
    let instructions = str.split(",");

    let mut path = Path {
        segments: Vec::new(),
    };

    let mut curr_cell = Cell {
        x: 0,
        y: 0,
    };

    for instruction in instructions {
        let mut segment = PathSegment {
            start_point: curr_cell.clone(),
            length: 0,
            orientation: Orientation::UpDown,
        };

        match instruction.chars().nth(0) {
            Some('R') => {
                segment.orientation = Orientation::LeftRight;
                segment.length = 1;
            }
            Some('L') => {
                segment.orientation = Orientation::LeftRight;
                segment.length = -1;
            }
            Some('U') => {
                segment.orientation = Orientation::UpDown;
                segment.length = 1;
            }
            Some('D') => {
                segment.orientation = Orientation::UpDown;
                segment.length = -1;
            }
            _ => return Err(format!("Failed to parse instruction: {}", instruction)),
        }

        match instruction.get(1..) {
            Some(length_str) => match length_str.parse::<i32>() {
                Ok(length) => segment.length *= length,
                Err(e) => return Err(format!("Failed to get length from: {}", length_str))
            }
            None => return Err(format!("Failed to get length from instruction: {}", instruction))
        }

        match segment.orientation {
            Orientation::UpDown => curr_cell.y += (segment.length),
            Orientation::LeftRight => curr_cell.x += (segment.length),
        }

        path.segments.push(segment)
    }

    return Ok(path);
}

fn segmentsIntersection(seg1: &PathSegment, seg2: &PathSegment) -> Option<Cell> {
// We're assuming that colinear wires don't overlap

    if seg1.orientation != seg2.orientation {
        struct UpdownAndLeftRightLine {
            up_down_line: PathSegment,
            left_right_line: PathSegment,
        }

        let lines = match seg1.orientation {
            Orientation::UpDown => UpdownAndLeftRightLine {
                up_down_line: seg1.clone(),
                left_right_line: seg2.clone(),
            },
            Orientation::LeftRight => UpdownAndLeftRightLine {
                up_down_line: seg2.clone(),
                left_right_line: seg1.clone(),
            },
        };

        let updown = lines.up_down_line;
        let leftright = lines.left_right_line;

        let updown_start_y = updown.start_point.y;
        let updown_end_y = updown_start_y + updown.length;
        let updown_max_y = max(updown_start_y, updown_end_y);
        let updown_min_y = min(updown_start_y, updown_end_y);
        if ((leftright.start_point.y > updown_min_y) && (leftright.start_point.y < updown_max_y)) {
            let leftright_start_x = leftright.start_point.x;
            let leftright_end_x = leftright_start_x + leftright.length;
            let leftright_max_x = max(leftright_start_x, leftright_end_x);
            let leftright_min_x = min(leftright_start_x, leftright_end_x);
            if ((updown.start_point.x > leftright_min_x) && (updown.start_point.x < leftright_max_x)) {
                return Some(Cell {
                    x: updown.start_point.x,
                    y: leftright.start_point.y,
                });
            }
        }
    }

    return None;
}

fn findIntersections(paths: Vec<&Path>) -> Vec<Intersection> {
    let mut intersections = Vec::new();

// NOTE: this will yield duplicate intersections
    for path1 in &paths {
        for path2 in &paths {
            if (path1 != path2) {
                for segment1 in &path1.segments {
                    for segment2 in &path2.segments {
                        match segmentsIntersection(segment1, segment2) {
                            Some(p) => intersections.push(Intersection {
                                cell: p,
                                path1: (*path1).clone(),
                                path2: (*path2).clone(),
                            }),
                            None => {}
                        }
                    }
                }
            }
        }
    }

    return intersections;
}

fn manhatten_distance(cell: &Cell) -> i32 {
    return cell.x.abs() + cell.y.abs();
}

fn closer_cell<'a>(cell1: &'a Cell, cell2: &'a Cell) -> &'a Cell {
    let dist_diff = manhatten_distance(&cell1) - manhatten_distance(&cell2);
    if dist_diff > 0 {
        cell2
    } else {
        cell1
    }
}

fn wireLengthToPoint(cell: &Cell, path: &Path) -> i32 {
    let mut total_length = 0;

    for segment in &path.segments {
        let mut length = segment.length;
        while (length > 0) {
            let curr_cell = match segment.orientation {
                Orientation::LeftRight => Cell {
                    x: segment.start_point.x + length,
                    y: segment.start_point.y,
                },
                Orientation::UpDown => Cell {
                    x: segment.start_point.x,
                    y: segment.start_point.y + length,
                },
            };
            if curr_cell == *cell {
                total_length += length;
                return total_length;
            }
            length -= 1;
        }
        total_length += segment.length.abs();
    }

    return total_length;
}

fn cumulativeWireLength(inter: &Intersection) -> i32 {
    wireLengthToPoint(&inter.cell, &inter.path1) +
        wireLengthToPoint(&inter.cell, &inter.path2)
}

fn shorter_cumulative_length<'a>(inter1: &'a Intersection, inter2: &'a Intersection) -> &'a Intersection {
    let dist_diff = cumulativeWireLength(&inter1) - cumulativeWireLength(&inter2);
    if dist_diff > 0 {
        inter2
    } else {
        inter1
    }
}

fn part1(f: File) -> Result<(), String> {
    let lines = read_lines(f);
    let wires_result: Vec<Result<Path, String>> = lines.iter().map(convert_str_to_path).collect();
    let wires: Vec<&Path> = wires_result.iter().filter_map(|result| match result {
        Ok(path) => Some(path),
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }).collect();

    if (wires_result.len() != wires.len()) {
        return Err(String::from("Some wires failed to parse!"));
    }

    let intersections = findIntersections(wires);
//    let closest_intersection = intersections.iter().fold(
//        None,
//        |current_closest_point, point| match current_closest_point {
//            Some(current_closest_point) => Some(closer_cell(current_closest_point, point)),
//            None => Some(point),
//        });
    let intersection_with_min_wire_length = intersections.iter().fold(
        None,
        |curr_best, intersection| match curr_best {
            None => Some(intersection),
            Some(best_so_far) => Some(
                shorter_cumulative_length(best_so_far, intersection)),
        },
    );

    match intersection_with_min_wire_length {
        Some(intersection) => println!("Closest intersection is: {:?} with a distance of {}!", intersection.cell, cumulativeWireLength(intersection)),
        None => println!("No intersections found!"),
    }


    return Ok(());
}

fn main() -> Result<(), String> {
    let filepath = "src/bin/day_3/part_1_puzzle_input";
    let file = File::open(filepath);

    match file {
        Ok(f) => part1(f),
        Err(e) => Err(String::from("Failed to open file"))
    }
}