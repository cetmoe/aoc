use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

#[derive(Debug, Clone, Copy, Eq)]
struct Vector {
    dx: isize,
    dy: isize,
}

impl Vector {
    fn to_direction(&self) -> Direction {
        match self {
            Vector { dx: 0, dy: -1 } => Direction::Up,
            Vector { dx: 0, dy: 1 } => Direction::Down,
            Vector { dx: -1, dy: 0 } => Direction::Left,
            Vector { dx: 1, dy: 0 } => Direction::Right,
            _ => panic!("Invalid vector"),
        }
    }

    fn base() -> Self {
        Vector { dx: 0, dy: 0 }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn within_bounds(&self, matrix: &Matrix<&str>) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < matrix.max_x() && self.y < matrix.max_y()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.dx + other.x,
            y: self.dy + other.y,
        }
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Point;

    fn add(self, other: (isize, isize)) -> Point {
        Point {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    fn get_point(&self, point: Point) -> Option<&T> {
        self.get(point.y as usize, point.x as usize)
    }

    fn max_x(&self) -> isize {
        self.data[0].len() as isize
    }

    fn max_y(&self) -> isize {
        self.data.len() as isize
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|row| row.iter())
    }
}

impl<T> FromIterator<Vec<T>> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Matrix {
            data: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl Direction {
    fn to_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector { dx: 0, dy: -1 },
            Direction::Down => Vector { dx: 0, dy: 1 },
            Direction::Left => Vector { dx: -1, dy: 0 },
            Direction::Right => Vector { dx: 1, dy: 0 },
        }
    }
}

fn change_direction(c: &str, dir: Direction) -> Vector {
    match (c, dir) {
        ("|", Down) => Down.to_vector(),
        ("|", Up) => Up.to_vector(),
        ("-", Left) => Left.to_vector(),
        ("-", Right) => Right.to_vector(),
        ("L", Down) => Right.to_vector(),
        ("L", Left) => Up.to_vector(),
        ("J", Down) => Left.to_vector(),
        ("J", Right) => Up.to_vector(),
        ("7", Up) => Left.to_vector(),
        ("7", Right) => Down.to_vector(),
        ("F", Up) => Right.to_vector(),
        ("F", Left) => Down.to_vector(),
        (_, _) => Vector { dx: 0, dy: 0 },
    }
}

fn main() {
    let array_2d: Matrix<&str> = include_str!("./input.txt")
        .split("\n")
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Matrix<&str>>();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut nodes = VecDeque::<(Point, Direction)>::new();
    let mut points: Vec<Point> = vec![];

    let start_pos = array_2d
        .iter()
        .enumerate()
        .find_map(|(i, &c)| {
            if c == "S" {
                return Some(Point {
                    x: (i % array_2d.max_x() as usize) as isize,
                    y: (i / array_2d.max_x() as usize) as isize,
                });
            }
            None
        })
        .unwrap();

    visited.insert(start_pos);
    nodes.push_front((start_pos, Up)); // Up is irrelevant doesnt do anything
    while !nodes.is_empty() {
        let current = nodes.pop_front().unwrap();
        let current_character = array_2d.get_point(current.0).unwrap();
        points.push(current.0);

        if current_character == &"S" {
            for direction in &[Up, Down, Left, Right] {
                let next_node = current.0 + direction.to_vector();

                if next_node.within_bounds(&array_2d)
                    && !visited.contains(&next_node)
                    && change_direction(&array_2d.get_point(next_node).unwrap(), *direction)
                        != Vector::base()
                {
                    nodes.push_back((next_node, *direction));
                    visited.insert(next_node);
                    break;
                }
            }
        } else {
            let direction = change_direction(current_character, current.1);
            let next_node = current.0 + direction;
            let next_character = &array_2d.get_point(next_node).unwrap();

            if next_node.within_bounds(&array_2d)
                && !visited.contains(&next_node)
                && change_direction(&next_character, direction.to_direction()) != Vector::base()
            {
                nodes.push_back((next_node, direction.to_direction()));
                visited.insert(next_node);
            }
        }
    }
    println!("{:?}", points.len() / 2);

    let mut sum = 0;
    let n = points.len();
    for i in 0..n {
        let next_index = (i + 1) % n;
        sum += (points[i].x * points[next_index].y) - (points[next_index].x * points[i].y);
    }
    
    println!("{:?}", ((sum.abs() / 2) as isize) - ((n / 2) as isize) + 1)
}
