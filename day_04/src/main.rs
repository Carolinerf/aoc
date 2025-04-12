use std::{
    fs::read_to_string,
    io::{self},
};

fn main() {
    let file_path: &str = "src/input.txt";
    let answer: i32 = d4p1(file_path).unwrap();
    println!("Answer: {}", answer);
}

fn read_file(file_path: &str) -> Vec<Vec<char>> {
    read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn d4p1(file_path: &str) -> io::Result<i32> {
    let map: Vec<Vec<char>> = read_file(file_path);
    let n: usize = map.len();
    let m: usize = map[0].len();
    let dx: Vec<i32> = vec![-1, -1, 0, 1, 1, 1, 0, -1];
    let dy: Vec<i32> = vec![0, 1, 1, 1, 0, -1, -1, -1];

    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            for k in 0..8 {
                let mut str: String = "".to_string();
                for t in 0..4 {
                    let ni = (i as i32) + t * dx[k];
                    let nj = (j as i32) + t * dy[k];

                    if 0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32 {
                        str += map[ni as usize][nj as usize].to_string().as_str();
                    }
                }
                if str == "XMAS" {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d4p1() {
        assert_eq!(d4p1("src/demo.txt").unwrap(), 18);
    }
}
