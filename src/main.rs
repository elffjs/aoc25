use std::fs;

fn day1_part1(input: &str) -> i64 {
    let mut password = 0;
    let mut position = 50;

    for line in input.lines() {
        let direction = &line[0..1];
        let clicks: i32 = line[1..].parse().unwrap();

        if direction == "R" {
            position += clicks;
        } else if direction == "L" {
            position -= clicks;
        }

        position = position.rem_euclid(100);
        if position == 0 {
            password += 1;
        }
    }

    return password;
}

fn day1_part2(input: &str) -> i64 {
    let mut password = 0;
    let mut position: i32 = 50;

    for line in input.lines() {
        let direction = &line[0..1];
        let clicks: i32 = line[1..].parse().unwrap();

        if direction == "R" {
            for _ in 0..clicks {
                position += 1;
                position = position.rem_euclid(100);
                if position == 0 {
                    password += 1;
                }
            }
        } else if direction == "L" {
            for _ in 0..clicks {
                position -= 1;
                position = position.rem_euclid(100);
                if position == 0 {
                    password += 1;
                }
            }
        }
    }

    return password;
}

fn day2_part1(input: &str) -> i64 {
    let mut invalid: i64 = 0;

    // Get rid of the trailing newline.
    for range in input.trim().split(",") {
        let v: Vec<i64> = range.split("-").map(|x| x.parse().unwrap()).collect();
        let low = v[0];
        let hi = v[1];

        for num in low..=hi {
            let s = num.to_string();
            let l: usize = s.len();

            if l % 2 != 0 {
                continue;
            }

            let block_len = l / 2;

            if &s[..block_len] == &s[block_len..] {
                invalid += num;
            }
        }
    }
    return invalid;
}

fn day2_part2(input: &str) -> i64 {
    let mut invalid: i64 = 0;

    // Get rid of the trailing newline.
    for range in input.trim().split(",") {
        let v: Vec<i64> = range.split("-").map(|x| x.parse().unwrap()).collect();
        let low = v[0];
        let hi = v[1];

        for num in low..=hi {
            let s = num.to_string();
            let l: usize = s.len();

            // Can't be more than half the size of the whole string.
            let biggest_block = l / 2;

            'block_size_loop: for block_len in 1..=biggest_block {
                if l % block_len != 0 {
                    continue;
                }

                let block_ct = l / block_len;

                let first = &s[0..block_len];

                for block_num in 1..block_ct {
                    let this = &s[block_num * block_len..(block_num + 1) * block_len];
                    if this != first {
                        continue 'block_size_loop;
                    }
                }

                invalid += num;
                break;
            }
        }
    }
    return invalid;
}

fn main() {
    let day1_input = fs::read_to_string("input1").unwrap();
    println!("D1P1: {}", day1_part1(&day1_input));
    println!("D1P2: {}", day1_part2(&day1_input));

    let day2_input = fs::read_to_string("input2").unwrap();
    println!("D2P1: {}", day2_part1(&day2_input));
    println!("D2P2: {}", day2_part2(&day2_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(3, day1_part1(input));
        assert_eq!(6, day1_part2(input));
    }

    #[test]
    fn test_day2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, day2_part1(input));
        assert_eq!(4174379265, day2_part2(input));
    }
}
