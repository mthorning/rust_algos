#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

const WALL: char = '#';

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn walk(
    maze: &[[char; 5]; 5],
    curr: Point,
    end: Point,
    path: &mut Vec<Point>,
    seen: &mut [[bool; 5]; 5],
) -> bool {

    // 1. if we're out of the maze
    if curr.y < 0
        || curr.x < 0
        || (curr.y as usize) >= maze.len()
        || (curr.x as usize) >= maze[0].len()
    {
        return false;
    }

    // 2. if we're on a wall
    if maze[curr.y as usize][curr.x as usize] == WALL {
        return false;
    }

    // 3. if we're on a square we've just visited
    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }

    // 4. if we're at the end
    if curr == end {
        path.push(end);
        return true;
    }

    seen[curr.y as usize][curr.x as usize] = true;

    path.push(Point {
        y: curr.y,
        x: curr.x,
    });


    for (y, x) in DIRS.iter() {
        let new_end = end.clone();
        let new_curr = Point {
            y: curr.y + y,
            x: curr.x + x,
        };

        if walk(maze, new_curr, new_end, path, seen) {
            return true;
        }
    }

    path.pop();

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_compare() {
        let a = Point{ x: 2, y: 3 };
        let b = Point{ x: 2, y: 3 };
        let c = Point{ x: 3, y: 3 };
        assert!(a == b);
        assert!(!(a == c));
    }

    #[test]
    fn it_returns_true_and_sets_path() {
        let maze = [
            ['#', '#', '#', ' ', '#'],
            ['#', ' ', '#', ' ', '#'],
            ['#', ' ', '#', ' ', '#'],
            ['#', ' ', ' ', ' ', '#'],
            ['#', ' ', '#', '#', '#'],
        ];

        let start = Point { y: 0, x: 3 };
        let end = Point { y: 4, x: 1 };
        let mut path = Vec::new();
        let mut seen = [[false; 5]; 5];

        let success = walk(&maze, start, end, &mut path, &mut seen);

        assert!(success);

        let correct_path = [
            Point { y: 0, x: 3 },
            Point { y: 1, x: 3 },
            Point { y: 2, x: 3 },
            Point { y: 3, x: 3 },
            Point { y: 3, x: 2 },
            Point { y: 3, x: 1 },
            Point { y: 4, x: 1 },
        ];

        println!("{:?}", path);
        for (i, point) in path.into_iter().enumerate() {
            assert_eq!(point, correct_path[i]);
        }
    }

    #[test]
    fn it_returns_false() {
        let maze = [
            ['#', '#', '#', ' ', '#'],
            ['#', ' ', '#', ' ', '#'],
            ['#', ' ', '#', ' ', '#'],
            ['#', ' ', '#', ' ', '#'],
            ['#', ' ', '#', '#', '#'],
        ];

        let start = Point { y: 0, x: 3 };
        let end = Point { y: 4, x: 1 };
        let mut path = Vec::new();
        let mut seen = [[false; 5]; 5];

        assert!(!walk(&maze, start, end, &mut path, &mut seen));
    }
}
