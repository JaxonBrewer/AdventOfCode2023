 use std::fs;

fn get_number(c: Vec<char>, i: usize) -> Option<char> {
    let ch = c[i];
    if ch == 'o' {
        if c.len() > i+2 && c[i+1] == 'n' && c[i+2] == 'e' {
            return Some('1');
        }
    } else if ch == 't' {
        if c.len() > i+2 && c[i+1] == 'w' && c[i+2] == 'o' {
            return Some('2');
        } else if c.len() > i+4 && c[i+1] == 'h' && c[i+2] == 'r' && c[i+3] == 'e' && c[i+4] == 'e' {
            return Some('3');
        }
    } else if ch == 'f' {
        if c.len() > i+3 && c[i+1] == 'o' && c[i+2] == 'u' && c[i+3] == 'r' {
            return Some('4');
        } else if c.len() > i+3 && c[i+1] == 'i' && c[i+2] == 'v' && c[i+3] == 'e' {
            return Some('5');
        }
    } else if ch == 's' {
        if c.len() > i+2 && c[i+1] == 'i' && c[i+2] == 'x' {
            return Some('6');
        } else if c.len() > i+4 && c[i+1] == 'e' && c[i+2] == 'v' && c[i+3] == 'e' && c[i+4] == 'n' {
            return Some('7');
        }
    } else if ch == 'e' {
        if c.len() > i+4 && c[i+1] == 'i' && c[i+2] == 'g' && c[i+3] == 'h' && c[i+4] == 't' {
            return Some('8');
        }
    } else if ch == 'n' {
        if c.len() > i+3 && c[i+1] == 'i' && c[i+2] == 'n' && c[i+3] == 'e' {
            return Some('9');
        }
    }
    None
}

const FILENAME: &str = "./src/input.txt";
fn main() -> std::io::Result<()> {
    // Read a file into a string
    let mut result: Vec<String> = Vec::new();
    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        result.push(line.to_string());
    }

    let mut values: Vec<(char, char)> = Vec::new();

    // iterate through every character in the string
    for l in result {
        let mut found_first: bool = false;
        let mut cur: (char, char) = (' ', ' ');
        let c: Vec<char> = l.chars().collect();

        for i in 0..c.len() {
            let mut found_num: Option<char> = None;
            let possible_num = get_number(c.clone(), i);
            let ch = c[i];

            if possible_num.is_some() {
                found_num = possible_num;
            } else if ch.is_digit(10) {
                found_num = Some(ch);
            }

            if found_num.is_some() {
                if !found_first {
                    cur.0 = found_num.unwrap();
                    cur.1 = found_num.unwrap();
                    found_first = true;
                } else {
                    cur.1 = found_num.unwrap();
                }
            }
        }
        values.push(cur);
    }

    let mut sum: i32 = 0;
    for i in values {
        // concatenate the two characters into a string
        let num_raw: String = format!("{}{}", i.0, i.1);
        println!("Found Raw: {}", num_raw);
        let num: i32 = num_raw.parse::<i32>().unwrap();
        sum += num;
    }

    println!("Sum: {}", sum);
    Ok(())
}