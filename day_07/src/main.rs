use std::{fs, io};

fn main() {
    let filepath = "src/input.txt";
    // ===== Part One =====
    let answer = d7p1(filepath).unwrap();
    println!("Part One Answer: {}", answer);

    // ===== Part Two =====
    let answer = d7p2(filepath).unwrap();
    println!("Part Two Answer: {}", answer);
}

fn d7p1(filepath: &str) -> io::Result<usize> {
    fn left_equal_right(nums: &[usize], ops: &[char]) -> usize {
        let mut sum = nums[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '+' => sum += nums[i + 1],
                '*' => sum *= nums[i + 1],
                _ => sum += 0,
            }
        }
        sum
    }

    fn generate_op_combs(n: usize) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        for s in 0..(1 << n) {
            let mut comb = Vec::with_capacity(n);
            for i in 0..n {
                if (s >> i) & 1 == 1 {
                    comb.push('*');
                } else {
                    comb.push('+');
                }
            }
            result.push(comb);
        }
        result
    }
    let mut total: usize = 0;
    let content = fs::read_to_string(filepath).unwrap();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: usize = parts[0].parse().unwrap();
        let numbers: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let op_combs = generate_op_combs(numbers.len() - 1);
        let mut matched = false;
        for ops in op_combs {
            if left_equal_right(&numbers, &ops) == target {
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

fn d7p2(filepath: &str) -> io::Result<usize> {
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

    fn generate_op_combs(n: usize) -> Vec<Vec<&'static str>> {
        let mut result = Vec::new();
        let ops = ["+", "*", "||"];
        for i in 0..(3_i32.pow(n as u32) as usize) {
            let mut comb = Vec::with_capacity(n);
            let mut index = i;
            for _ in 0..n {
                comb.push(ops[index % 3]);
                index /= 3;
            }
            result.push(comb);
        }
        result
    }
    let mut total: usize = 0;
    let content = fs::read_to_string(filepath).unwrap();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: usize = parts[0].parse().unwrap();
        let numbers: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let op_combs = generate_op_combs(numbers.len() - 1);
        let mut matched = false;
        for ops in op_combs {
            if left_equal_right(&numbers, &ops) == target {
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
