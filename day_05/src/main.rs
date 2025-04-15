use std::{collections::HashMap, fs, io};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d5p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
    let answer = d5p2(filepath).unwrap();
    println!("Part Two Answer: {}", answer);
}

fn d5p1(filepath: &str) -> io::Result<i32> {
    let input = fs::read_to_string(filepath).expect("Failed to read file.");
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    // println!("{}", parts.len());
    let rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Vec<&str> = line.trim().split('|').collect();
            let x = nums[0].parse::<i32>().expect("Invalid integer in rule");
            let y = nums[1].parse::<i32>().expect("Invalid integer in rule");
            (x, y)
        })
        .collect();

    let mut total = 0;
    for update_line in parts[1].lines().filter(|line| !line.trim().is_empty()) {
        let pages: Vec<i32> = update_line
            .split(',')
            .map(|num| {
                num.trim()
                    .parse::<i32>()
                    .expect("Invalid integer in update")
            })
            .collect();

        let mut pos = std::collections::HashMap::new();
        for (i, page) in pages.iter().enumerate() {
            pos.insert(page, i);
        }

        let mut vaild = true;
        for (x, y) in &rules {
            if let (Some(&pos_x), Some(&pos_y)) = (pos.get(x), pos.get(y)) {
                if pos_x >= pos_y {
                    vaild = false;
                    break;
                }
            }
        }

        if vaild {
            total += pages[pages.len() / 2];
        }
    }
    Ok(total)
}

fn d5p2(filepath: &str) -> io::Result<i32> {
    let input = fs::read_to_string(filepath).expect("Failed to read file.");
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    // println!("{}", parts.len());
    let rules: Vec<(i32, i32)> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let nums: Vec<&str> = line.trim().split('|').collect();
            let x = nums[0].parse::<i32>().expect("Invalid integer in rule");
            let y = nums[1].parse::<i32>().expect("Invalid integer in rule");
            (x, y)
        })
        .collect();

    let mut total = 0;
    for update_line in parts[1].lines().filter(|line| !line.trim().is_empty()) {
        let pages: Vec<i32> = update_line
            .split(',')
            .map(|num| {
                num.trim()
                    .parse::<i32>()
                    .expect("Invalid integer in update")
            })
            .collect();

        let mut pos = std::collections::HashMap::new();
        for (i, page) in pages.iter().enumerate() {
            pos.insert(page, i);
        }

        let mut vaild = true;
        for (x, y) in &rules {
            if let (Some(&pos_x), Some(&pos_y)) = (pos.get(x), pos.get(y)) {
                if pos_x >= pos_y {
                    vaild = false;
                    break;
                }
            }
        }

        if vaild {
            continue;
        }

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegree: HashMap<i32, i32> = HashMap::new();

        for page in &pages {
            graph.insert(*page, Vec::new());
            indegree.insert(*page, 0);
        }

        for (x, y) in &rules {
            if graph.contains_key(x) && graph.contains_key(y) {
                graph.get_mut(x).unwrap().push(*y);
                *indegree.get_mut(y).unwrap() += 1;
            }
        }

        let mut available: Vec<i32> = indegree
            .iter()
            .filter_map(|(&page, &deg)| if deg == 0 { Some(page) } else { None })
            .collect();
        available.sort_unstable_by(|a, b| b.cmp(a));

        let mut fixed_order = Vec::with_capacity(pages.len());
        while let Some(node) = available.pop() {
            fixed_order.push(node);
            if let Some(neighbors) = graph.get(&node) {
                for &nbr in neighbors {
                    let deg = indegree.get_mut(&nbr).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        available.push(nbr);
                    }
                }
                available.sort_unstable_by(|a, b| b.cmp(a));
            }
        }
        if fixed_order.len() != pages.len() {
            continue;
        }
        total += fixed_order[fixed_order.len() / 2];
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d5p1() {
        assert_eq!(d5p1("src/demo.txt").unwrap(), 143);
    }

    #[test]
    fn test_d5p2() {
        assert_eq!(d5p2("src/demo.txt").unwrap(), 123);
    }
}
