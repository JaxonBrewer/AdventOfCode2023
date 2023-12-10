use std::fs;

const FILENAME: &str = "input.txt";

fn main() {
    let matrix = read_file(FILENAME.to_string());
    let mut sum = 0;
    for i in 0..matrix.len() {
        let mut start_index: Option<usize> = None;
        let mut end_index: Option<usize> = None;
        for j in 0..matrix[i].len() {
            let col = matrix[i][j];
            if col.is_digit(10) {
                if start_index != None {
                    end_index = Some(j);
                } else {
                    start_index = Some(j);
                    end_index = Some(j);
                }
            } else {
                if start_index != None {
                    // is usable number
                    sum += get_value(&matrix, i, start_index.unwrap(), end_index.unwrap());
                    start_index = None;
                    end_index = None;
                }
            }
        }
        if start_index != None {
            // is usable number
            sum += get_value(&matrix, i, start_index.unwrap(), end_index.unwrap());
        }
    }

    println!("Sum: {}", sum);
}

fn get_value(matrix: &Vec<Vec<char>>, row: usize, col_start: usize, col_end: usize) -> i32 {
    let mut result = 0;
    let value = matrix[row][col_start..col_end + 1]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();

    let start_index = if col_start == 0 { 0 } else { col_start - 1 };
    let end_index = if col_end == matrix[row].len() - 1 {
        col_end
    } else {
        col_end + 1
    };

    // look for symbol above
    if row > 0 {
        for c in matrix[row - 1][start_index..end_index + 1].iter() {
            if !c.is_digit(10) && c != &'.' {
                result = value;
            }
        }
    }

    // look for symbol below
    if row < matrix.len() - 1 {
        for c in matrix[row + 1][start_index..end_index + 1].iter() {
            if !c.is_digit(10) && c != &'.' {
                result = value;
            }
        }
    }

    // look for symbol left
    let mut c = matrix[row][start_index];
    if !c.is_digit(10) && c != '.' {
        result = value;
    }

    // look for symbol right
    c = matrix[row][end_index];
    if !c.is_digit(10) && c != '.' {
        result = value;
    }
    result
}

fn read_file(path: String) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }
    result
}
