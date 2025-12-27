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

    for turn in parse(input) {
        let sign = match turn.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        position += sign * turn.clicks;

        if position % 100 == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &str) -> i32 {
    let mut password = 0;
    let mut position: i32 = 50;

    for turn in parse(input) {
        let step = match turn.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        // TODO: We can surely avoid performing all these steps, but I
        // worry that there will be a lot of cases. It seems to be fast enough
        // to move on.
        for _ in 0..turn.clicks {
            position += step;
            if position % 100 == 0 {
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
