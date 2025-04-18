use std::{collections::HashSet, fs, io};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d7p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
    let answer = d7p2(filepath).unwrap();
    println!("Part Two Answer: {}", answer);
}

fn matches_target(nums: &[usize], target: usize, ops: &[&str]) -> bool {
    let mut curr: HashSet<usize> = HashSet::new();
    curr.insert(nums[0]);

    for &num in &nums[1..] {
        let mut next = HashSet::new();
        for &val in &curr {
            for &op in ops {
                let new_val = match op {
                    "+" => val + num,
                    "*" => val * num,
                    "||" => {
                        let s = format!("{}{}", val, num);
                        s.parse().unwrap()
                    }
                    _ => continue,
                };
                if new_val > target {
                    continue;
                }
                if new_val == target && num == *nums.last().unwrap() {
                    return true;
                }
                next.insert(new_val);
            }
        }
        curr = next;
        if curr.is_empty() {
            return false;
        }
    }
    curr.contains(&target)
}

fn solve(filepath: &str, ops: Vec<&str>) -> io::Result<usize> {
    let mut total = 0;
    let content = fs::read_to_string(filepath)?;
    for line in content.lines() {
        let mut parts = line.split(':');
        let target: usize = parts.next().unwrap().parse().unwrap();
        let nums: Vec<usize> = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if matches_target(&nums, target, &ops) {
            total += target;
        }
    }
    Ok(total)
}

fn d7p1(filepath: &str) -> io::Result<usize> {
    solve(filepath, vec!["+", "*"])
}

fn d7p2(filepath: &str) -> io::Result<usize> {
    solve(filepath, vec!["+", "*", "||"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d7p1() {
        assert_eq!(d7p1("src/demo.txt").unwrap(), 3749);
    }

    #[test]
    fn test_d7p2() {
        assert_eq!(d7p2("src/demo.txt").unwrap(), 11387);
    }
}
