use std::cmp::{max, min};

const EXPANSION_RATE: usize = 1000000 - 1;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn distance(mut p1: Point, mut p2: Point, xind: &Vec<usize>, yind: &Vec<usize>) -> usize {
    for x in min(p1.x, p2.x)..max(p1.x, p2.x) {
        if xind.contains(&x) {
            if p1.x > p2.x {
                p1.x += EXPANSION_RATE;
            } else {
                p2.x += EXPANSION_RATE;
            }
        }
    }

    for y in min(p1.y, p2.y)..max(p1.y, p2.y) {
        if yind.contains(&y) {
            if p1.y > p2.y {
                p1.y += EXPANSION_RATE;
            } else {
                p2.y += EXPANSION_RATE;
            }
        }
    }

    ((p1.x as isize - p2.x as isize).abs() + (p1.y as isize - p2.y as isize).abs()) as usize
}

fn main() {
    let map = include_str!("./input.txt")
        .lines()
        .map(|line| line.split("").filter(|c| c != &"").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let y_indexs = map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|c| c == &".") {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let transposed_map = transpose(map.clone());
    let x_indexs = transposed_map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|c| c == &".") {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let points = map
        .iter()
        .enumerate()
        .filter_map(|(y, arr)| {
            let line_points = arr
                .iter()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == &"#" {
                        Some(Point::new(x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>();
            if line_points.is_empty() {
                None
            } else {
                Some(line_points)
            }
        })
        .flatten()
        .collect::<Vec<Point>>();

    let distances: usize = points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, x)| {
            points[i + 1..]
                .iter()
                .copied()
                .map(|y| distance(x, y, &x_indexs.clone(), &y_indexs.clone()))
                .collect::<Vec<usize>>()
        })
        .sum();

    println!("{:?}", distances);
}
