use std::{fs, io};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d5p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_d5p1() {
        assert_eq!(d5p1("src/demo.txt").unwrap(), 143);
    }
}
