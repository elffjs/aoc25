#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Turn {
    direction: Direction,
    clicks: i32,
}

fn parse(input: &str) -> Vec<Turn> {
    input
        .lines()
        .map(|l| Turn {
            direction: match l.chars().next().unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!(),
            },
            clicks: l[1..].parse().unwrap(),
        })
        .collect()
}

pub fn part1(input: &str) -> i32 {
    let mut password = 0;
    let mut position: i32 = 50;

    for Turn { direction, clicks } in parse(input) {
        position += match direction {
            Direction::Left => 1,
            Direction::Right => -1,
        } * clicks;

        if position % 100 == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(INPUT));
    }
}
