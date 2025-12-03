use std::fs;

fn day1_part1(input: &str) -> i32 {
    let mut password = 0;
    let mut position: i32 = 50;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let clicks: i32 = line[1..].parse().unwrap();

        match direction {
            'R' => position += clicks,
            'L' => position -= clicks,
            _ => panic!("Unexpected direction {}", direction),
        }

        position = position % 100;
        if position == 0 {
            password += 1;
        }
    }

    password
}

fn day1_part2(input: &str) -> i32 {
    let mut password = 0;
    let mut position: i32 = 50;

    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let clicks: i32 = line[1..].parse().unwrap();

        let step = match direction {
            'R' => 1,
            'L' => -1,
            _ => panic!("Unexpected direction {}", direction),
        };

        for _ in 0..clicks {
            position += step;
            position = position % 100;
            if position == 0 {
                password += 1;
            }
        }
    }

    password
}

fn day2_part1(input: &str) -> i64 {
    let mut invalid: i64 = 0;

    // Get rid of the trailing newline.
    for range in input.trim().split(",") {
        let [low, hi] = range
            .split("-")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for num in low..=hi {
            let s = num.to_string();
            let len: usize = s.len();

            if len % 2 != 0 {
                continue;
            }

            let block_len = len / 2;

            if s[..block_len] == s[block_len..] {
                invalid += num;
            }
        }
    }

    invalid
}

fn day2_part2(input: &str) -> i64 {
    let mut invalid: i64 = 0;

    // Get rid of the trailing newline.
    for range in input.trim().split(",") {
        let [low, hi] = range
            .split("-")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        for num in low..=hi {
            let s = num.to_string();
            let len: usize = s.len();

            // Can't be more than half the size of the whole string.
            let biggest_block = len / 2;

            'block_size_loop: for block_len in 1..=biggest_block {
                if len % block_len != 0 {
                    continue;
                }

                let block_ct = len / block_len;
                let first = &s[..block_len];

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

    invalid
}

fn day3_part1(input: &str) -> i64 {
    let mut joltage: i64 = 0;

    for line in input.lines() {
        let mut max: i64 = -1;

        for i in 0..line.len() - 1 {
            for j in i + 1..line.len() {
                let together = format!("{}{}", &line[i..i + 1], &line[j..j + 1]);
                let val: i64 = together.parse().unwrap();

                if val > max {
                    max = val;
                }
            }
        }

        joltage += max;
    }

    joltage
}

fn day3_part2(input: &str) -> i64 {
    let mut joltage: i64 = 0;

    for line in input.lines() {
        let mut digit_idxs: [usize; 12] = [0; 12];
        for digit_index in 0..12 {
            let start = if digit_index == 0 {
                0
            } else {
                digit_idxs[digit_index - 1] + 1
            };
            let stop = line.len() - 11 + digit_index;

            let mut max_ind = start;
            for i in start + 1..stop {
                let max_val = line[max_ind..max_ind + 1].parse::<i64>().unwrap();
                let val = line[i..i + 1].parse::<i64>().unwrap();

                if val > max_val {
                    max_ind = i;
                }
            }

            digit_idxs[digit_index] = max_ind;
        }

        let mut out: String = String::new();
        for digit_index in 0..12 {
            out = format!(
                "{}{}",
                out,
                &line[digit_idxs[digit_index]..digit_idxs[digit_index] + 1]
            );
        }

        joltage += out.parse::<i64>().unwrap();
    }

    joltage
}

fn main() {
    let day1_input = fs::read_to_string("input1").unwrap();
    println!("D1P1: {}", day1_part1(&day1_input));
    println!("D1P2: {}", day1_part2(&day1_input));

    let day2_input = fs::read_to_string("input2").unwrap();
    println!("D2P1: {}", day2_part1(&day2_input));
    println!("D2P2: {}", day2_part2(&day2_input));

    let day3_input = fs::read_to_string("input3").unwrap();
    println!("D3P1: {}", day3_part1(&day3_input));
    println!("D3P2: {}", day3_part2(&day3_input));
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

    #[test]
    fn test_day3() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, day3_part1(input));
        assert_eq!(3121910778619, day3_part2(input));
    }
}
