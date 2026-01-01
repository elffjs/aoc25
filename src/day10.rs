#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

fn subsets<T: Copy>(set: Vec<T>, size: usize) -> Vec<Vec<T>> {
    match size {
        0 => vec![vec![]],
        1 => set.iter().map(|x| vec![x.clone()]).collect(),
        _ => {
            let mut out = Vec::new();
            
            if set.is_empty() {
                return out;
            }

            for e in subsets((&set[1..]).to_vec(), size) {
                out.push(e);
            }

            if size > 0 {
            for mut e in subsets((&set[1..]).to_vec(), size-1) {
                e.insert(0, set[0]);
                out.push(e);
            }
        }

            out
        },
    }
}

// fn power_set_aux

pub fn part1(input: &str) -> i64 {
    let machines: Vec<Machine> = input.lines().map(|line| {
        let v: Vec<&str> = line.split_whitespace().collect();

        let lights_str = v[0][1..v[0].len()-1].to_string();
        let lights = lights_str.chars().map(|c|  match c {
            '#' => true,
            '.' => false,
            _ => panic!(),
        }).collect();

        let buttons: Vec<Vec<usize>> = v[1..v.len()-1].iter().map(|button_raw| {
            button_raw[1..button_raw.len()-1].split(',').map(|ps| ps.parse().unwrap()).collect()
        }).collect();
        Machine { diagram: lights, buttons }
    }).collect();
    println!("{:?}", machines);

    let mut tot: i64 = 0;

    'machineLoop:
    for machine in machines {

        for button_count in 0..machine.buttons.len() {
            let button_indices: Vec<usize> = (0..machine.buttons.len()).collect();
            for button_set in subsets(button_indices, button_count) {
                let mut state = vec![false; machine.diagram.len()];

                for button_idxs in button_set {
                    for pos in &machine.buttons[button_idxs] {
                        state[*pos] = !state[*pos];
                    }
                }

                if state == machine.diagram {
                    tot += button_count as i64;
                    continue 'machineLoop;
                }
            }
        }
    }

    tot
}


#[cfg(test)]
mod tests {
    use super::*;
        #[test]
    fn test_day10() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        println!("XDD {:?}", subsets(vec![1, 2, 3, 4], 2));

        assert_eq!(7, part1(input));
    }
}