use std::{fs, io, vec};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d7p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
    let answer = d7p2(filepath).unwrap();
    println!("Part Two Answer: {}", answer);
}

fn left_equal_right(nums: &[usize], ops: &[&str]) -> usize {
    let mut sum = nums[0];
    for (i, op) in ops.iter().enumerate() {
        sum = match *op {
            "+" => sum + nums[i + 1],
            "*" => sum * nums[i + 1],
            "||" => {
                let concat = format!("{}{}", sum, nums[i + 1]);
                concat.parse::<usize>().unwrap()
            }
            _ => sum + 0,
        }
    }
    sum
}

fn generate_op_combs(n: usize, ops: Vec<&str>) -> Vec<Vec<&str>> {
    let mut result = Vec::new();
    let b = ops.len();
    for i in 0..(b.pow(n as u32) as usize) {
        let mut comb = Vec::with_capacity(n);
        let mut index = i;
        for _ in 0..n {
            comb.push(ops[index % b]);
            index /= b;
        }
        result.push(comb);
    }
    result
}

fn solve(filepath: &str, ops: Vec<&str>) -> io::Result<usize> {
    let mut total: usize = 0;
    let content = fs::read_to_string(filepath).unwrap();
    let mut op_combs_by_n = Vec::new();
    for i in 1..12 {
        op_combs_by_n.push(generate_op_combs(i, ops.clone()));
    }

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: usize = parts[0].parse().unwrap();
        let numbers: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let op_combs = op_combs_by_n[numbers.len() - 2].clone();
        let mut matched = false;
        for comb in op_combs {
            if left_equal_right(&numbers, &comb) == target {
                matched = true;
                break;
            }
        }
        if matched {
            total += target;
        }
    }
    Ok(total)
}

fn d7p1(filepath: &str) -> io::Result<usize> {
    Ok(solve(filepath, vec!["+", "*"]).unwrap())
}

fn d7p2(filepath: &str) -> io::Result<usize> {
    Ok(solve(filepath, vec!["+", "*", "||"]).unwrap())
}

#[cfg(test)]
mod test {
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
