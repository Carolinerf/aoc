use std::collections::HashSet;
use std::{fs, io};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d6p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
    let answer = d6p2(filepath).unwrap();
    println!("Part One Answer: {}", answer);
}

fn read_file(filepath: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn d6p1(filepath: &str) -> io::Result<usize> {
    let map = read_file(filepath);
    let n = map.len();
    let m = map[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let direction = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut x = 0;
    let mut y = 0;
    let mut k = 0;

    for i in 0..n {
        for j in 0..m {
            match map[i][j] {
                '^' => {
                    x = i;
                    y = j;
                }
                _ => {}
            }
        }
    }

    visited.insert((x, y));
    loop {
        let (dx, dy) = direction[k];
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if 0 <= nx && nx < n as i32 && 0 <= ny && ny < m as i32 {
            if map[nx as usize][ny as usize] == '#' {
                k = (k + 1) % 4;
            } else {
                x = nx as usize;
                y = ny as usize;
                visited.insert((x, y));
            }
        } else {
            break;
        }
    }

    Ok(visited.len())
}

fn bfs(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let mut x = start.0;
    let mut y = start.1;
    let mut k = 0;

    let n = map.len();
    let m = map[0].len();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();
    let direction = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    visited.insert((x, y, k));

    loop {
        let (dx, dy) = direction[k];
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if !(0 <= nx && nx < n as i32 && 0 <= ny && ny < m as i32) {
            return false;
        }

        let (nx, ny) = (nx as usize, ny as usize);
        if map[nx][ny] == '#' {
            k = (k + 1) % 4;
        } else {
            x = nx;
            y = ny;
        }

        if !visited.insert((x, y, k)) {
            return true;
        }
    }
}

fn d6p2(filepath: &str) -> io::Result<usize> {
    let mut map = read_file(filepath);
    let n = map.len();
    let m = map[0].len();

    let mut start = (0, 0);
    for i in 0..n {
        for j in 0..m {
            match map[i][j] {
                '^' => {
                    start = (i, j);
                }
                _ => {}
            }
        }
    }

    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '.' && (i, j) != start {
                map[i][j] = '#';
                if bfs(&map, start) {
                    total += 1;
                }
                map[i][j] = '.';
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d6p1() {
        assert_eq!(d6p1("src/demo.txt").unwrap(), 41);
    }

    #[test]
    fn test_d6p2() {
        assert_eq!(d6p2("src/demo.txt").unwrap(), 6);
    }
}
