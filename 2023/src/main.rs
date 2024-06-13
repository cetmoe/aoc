use std::vec;

const VALID_CHARS: &[char] = &['|', '-', 'L', 'J', '7', 'F'];

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let array_2d: Vec<Vec<&str>> = include_str!("./input.txt")
        .split("\n")
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect();

    let mut dir = 1;

    for i in 0..array_2d.len() {
        for j in 0..array_2d[0].len() {
            let c = array_2d[i][j];
            if c == "S" {
                let initial_pos = Point {
                    x: i as isize,
                    y: j as isize,
                };

                let mut starting_positions: Vec<Point> = vec![];
                for next_x in -1..1 {
                    for next_y in -1..1 {
                        let x = initial_pos.x + next_x;
                        let y = initial_pos.y + next_y;

                        if array_2d[x as usize][y as usize]
                            .chars()
                            .all(|c| VALID_CHARS.contains(&c))
                        {
                            starting_positions.push(Point {
                                x: next_x,
                                y: next_y,
                            });
                        }
                    }
                }
                println!("{:?}", starting_positions);
            }
        }
    }
}

fn change_dir(c: &str, dir: Direction) -> Point {
    match (c, dir) {
        ("|", Direction::Down) => Point { x: 0, y: -1 },
        ("|", Direction::Up) => Point { x: 0, y: 1 },
        ("-", Direction::Left) => Point { x: -1, y: 0 },
        ("-", Direction::Right) => Point { x: 1, y: 0 },
        ("L", Direction::Down) => Point { x: 1, y: -1 },
        ("L", Direction::Left) => Point { x: -1, y: 1 },
        ("J", Direction::Down) => Point { x: -1, y: -1 },
        ("J", Direction::Right) => Point { x: 1, y: 1 },
        ("7", Direction::Up) => Point { x: 1, y: -1 },
        ("7", Direction::Right) => Point { x: -1, y: 1 },
        ("F", Direction::Up) => Point { x: 1, y: 1 },
        ("F", Direction::Left) => Point { x: -1, y: -1 },
        (_, _) => Point { x: 0, y: 0 },
    }
}
