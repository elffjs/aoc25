use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    target_voltages: Vec<usize>,
}

#[derive(PartialEq, Debug)]
struct Matrix {
    entries: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new_zeroed(rows: usize, cols: usize) -> Self {
        Self {
            entries: vec![0; rows * cols],
            rows,
            cols,
        }
    }

    fn from_rows(rows: Vec<Vec<i32>>) -> Self {
        let row_ct = rows.len();
        let col_ct = if row_ct == 0 { 0 } else { rows[0].len() };

        let entries = rows.iter().flatten().cloned().collect();

        Self {
            entries,
            rows: row_ct,
            cols: col_ct,
        }
    }

    fn get(self: &Self, row: usize, col: usize) -> i32 {
        if row >= self.rows || col >= self.cols {
            panic!("out of bounds");
        } else {
            self.entries[self.entry_idx(row, col)]
        }
    }

    fn entry_idx(self: &Self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn set(self: &mut Self, row: usize, col: usize, value: i32) {
        if row >= self.rows || col >= self.cols {
            panic!("out of bounds");
        }
        let idx = self.entry_idx(row, col);
        self.entries[idx] = value;
    }

    fn swap_rows(self: &mut Self, row1: usize, row2: usize) {
        if row1 >= self.rows || row2 >= self.rows {
            panic!("out of bounds")
        }

        if row1 == row2 {
            return;
        }

        for col in 0..self.cols {
            let row1_idx = self.entry_idx(row1, col);
            let row2_idx = self.entry_idx(row2, col);

            self.entries.swap(row1_idx, row2_idx);
        }
    }

    fn add_row_multiple(self: &mut Self, target_row: usize, source_row: usize, factor: i32) {
        if target_row >= self.rows || source_row >= self.rows {
            panic!("out of bounds")
        }

        for col in 0..self.cols {
            let target_idx = self.entry_idx(target_row, col);
            let source_idx = self.entry_idx(source_row, col);
            self.entries[target_idx] += factor * self.entries[source_idx];
        }
    }

    fn negate_row(self: &mut Self, row: usize) {
        if row >= self.rows {
            panic!("out of bounds")
        }

        for col in 0..self.cols {
            let idx = self.entry_idx(row, col);
            self.entries[idx] *= -1;
        }
    }

    // Make sure the leading entry, if there is one, is positive; and that the
    // there is no g.c.d.
    fn normalize_row(self: &mut Self, row: usize) {
        if row >= self.rows {
            panic!("out of bounds");
        }

        // Very wasteful. Should do Euclidean algorithm.
        let max_abs = (0..self.cols)
            .map(|col| self.get(row, col).abs())
            .max()
            .unwrap();

        'div_check: for d in (2..=max_abs).rev() {
            for col in 0..self.cols {
                if self.entries[row * self.cols + col] % d != 0 {
                    continue 'div_check;
                }
            }
            for col in 0..self.cols {
                self.entries[row * self.cols + col] /= d;
            }
            break;
        }

        if let Some(first_nontriv_col) = (0..self.cols).find(|&col| self.get(row, col) != 0) {
            if self.get(row, first_nontriv_col) < 0 {
                self.negate_row(row);
            }
        }
    }

    fn print(self: &Self) {
        if self.entries.is_empty() {
            return;
        }

        let max_width = self
            .entries
            .iter()
            .map(|x| x.to_string().len())
            .max()
            .unwrap()
            + 1;

        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{:1$}", self.get(row, col), max_width);
            }
            println!();
        }
    }
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
                for mut e in subsets((&set[1..]).to_vec(), size - 1) {
                    e.insert(0, set[0]);
                    out.push(e);
                }
            }

            out
        }
    }
}

// fn power_set_aux

pub fn part1(input: &str) -> i64 {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();

            let lights_str = v[0][1..v[0].len() - 1].to_string();
            let lights = lights_str
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                })
                .collect();

            let buttons: Vec<Vec<usize>> = v[1..v.len() - 1]
                .iter()
                .map(|button_raw| {
                    button_raw[1..button_raw.len() - 1]
                        .split(',')
                        .map(|ps| ps.parse().unwrap())
                        .collect()
                })
                .collect();

            let voltages_str = v[v.len() - 1][1..v[v.len() - 1].len() - 1].to_string();

            let voltages = voltages_str
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            Machine {
                diagram: lights,
                buttons: buttons,
                target_voltages: voltages,
            }
        })
        .collect();

    let mut tot: i64 = 0;

    'machineLoop: for machine in machines {
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

pub fn part2(input: &str) -> i64 {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| {
            let v: Vec<&str> = line.split_whitespace().collect();

            let lights_str = v[0][1..v[0].len() - 1].to_string();
            let lights = lights_str
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                })
                .collect();

            let buttons: Vec<Vec<usize>> = v[1..v.len() - 1]
                .iter()
                .map(|button_raw| {
                    button_raw[1..button_raw.len() - 1]
                        .split(',')
                        .map(|ps| ps.parse().unwrap())
                        .collect()
                })
                .collect();

            let voltages_str = v[v.len() - 1][1..v[v.len() - 1].len() - 1].to_string();

            let voltages = voltages_str
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            Machine {
                diagram: lights,
                buttons: buttons,
                target_voltages: voltages,
            }
        })
        .collect();

    let mut tot_press = 0;

    for (idx, machine) in machines.iter().enumerate() {
        let mut mx = Matrix::new_zeroed(machine.diagram.len(), machine.buttons.len() + 1);

        for (but_ind, button_config) in machine.buttons.iter().enumerate() {
            for pos in button_config {
                mx.set(*pos, but_ind, 1);
            }
        }

        for (eqn, volt) in machine.target_voltages.iter().enumerate() {
            mx.set(eqn, machine.buttons.len(), *volt as i32);
        }

        println!("Machine {}", idx);
        println!("Input");
        mx.print();

        reduce(&mut mx);

        println!("Reduced");

        mx.print();

        let free_vars = get_free_vars(&mx);
        println!("Free vars {:?}", free_vars);

        let mut free_var_limits: Vec<usize> = Vec::new();
        for free_var in &free_vars {
            free_var_limits.push(
                machine.buttons[*free_var]
                    .iter()
                    .map(|&pos| machine.target_voltages[pos])
                    .min()
                    .unwrap(),
            );
        }

        println!("Limits {:?}", free_var_limits);

        let mut queue: VecDeque<Vec<usize>> = VecDeque::new();
        let init = vec![0; free_vars.len()];

        queue.push_back(init);

        let mut best_so_far: Option<usize> = None;

        let mut config = vec![0; free_vars.len()];

        'outer: loop {
            let mut pos = free_vars.len() - 1;

            loop {
                config[pos] += 1;

                if config[pos] <= free_var_limits[pos] {
                    break;
                }

                config[pos] = 0;
                if pos == 0 {
                    break 'outer;
                }

                pos -= 1;
            }

            println!("COnfig {:?}", config);

            let mut input: Vec<Option<i32>> = vec![None; machine.buttons.len()];

            for i in 0..free_vars.len() {
                input[free_vars[i]] = Some(config[i] as i32);
            }

            let vec = assign_frees(&mx, input);

            if vec.iter().all(|c| !c.is_negative()) {
                let loc_tot_press = vec.iter().sum::<i32>() as usize;

                println!("Soln {:?} ? {} Presses", vec, loc_tot_press);

                match best_so_far {
                    None => best_so_far = Some(loc_tot_press),
                    Some(best) => best_so_far = Some(best.min(loc_tot_press)),
                }
            }

            for pos in 0..config.len() {
                let mut cc = config.clone();
                cc[pos] += 1;
                queue.push_back(cc);
            }
        }

        println!("TOT P {:?}", best_so_far);
        tot_press += best_so_far.expect("Nothing found?");
    }
    tot_press as i64
}

fn reduce(mx: &mut Matrix) {
    // Want to establish a pivot on this row.
    let mut work_row = 0;

    // Last column is for the image.
    'col_loop: for col in 0..mx.cols {
        loop {
            let mut row_to_move: Option<usize> = None;

            for row in work_row..mx.rows {
                if mx.get(row, col) != 0 {
                    match row_to_move {
                        None => {
                            row_to_move = Some(row);
                        }
                        Some(existing) => {
                            if mx.get(row, col).abs() < mx.get(existing, col).abs() {
                                row_to_move = Some(row);
                            }
                        }
                    }
                }
            }

            if let Some(row_yes) = row_to_move {
                if row_yes != work_row {
                    mx.swap_rows(work_row, row_yes);
                }
            } else {
                // Nothing interesting in this column.
                continue 'col_loop;
            }

            mx.normalize_row(work_row);

            let pivot = mx.get(work_row, col);

            // Divide everything below, leaving numbers less in size than the pivot.
            for row in work_row + 1..mx.rows {
                if mx.get(row, col) != 0 {
                    let quot = mx.get(row, col).div_euclid(pivot);
                    mx.add_row_multiple(row, work_row, -quot);
                }
            }

            // If anything nonzero remaining, need to swap one of these rows up and do this again.
            // This process must terminate eventually.
            if (work_row + 1..mx.rows).any(|row| mx.get(row, col) != 0) {
                continue;
            }

            // Divide above to lower values.
            // This may result in some common divisors that weren't there before.
            for row in 0..work_row {
                if mx.get(row, col) != 0 {
                    let quot = mx.get(row, col).div_euclid(pivot);

                    mx.add_row_multiple(row, work_row, -quot);
                    mx.normalize_row(row);
                }
            }

            break;
        }
        work_row += 1;
    }
}

fn get_free_vars(mx: &Matrix) -> Vec<usize> {
    let mut pivot_cols: Vec<usize> = Vec::new();

    for row in 0..mx.rows {
        let pivot_col = (0..mx.cols).find(|&col| mx.get(row, col) != 0);

        if let Some(c) = pivot_col {
            pivot_cols.push(c);
        }
    }

    (0..mx.cols - 1)
        .filter(|c| !pivot_cols.contains(c))
        .collect()
}

// Returns complete vector.
fn assign_frees(mx: &Matrix, assigns: Vec<Option<i32>>) -> Vec<i32> {
    assert_eq!(assigns.len(), mx.cols - 1);
    let mut pivots: Vec<(usize, usize)> = Vec::new();

    let mut pivot_col_to_row: HashMap<usize, usize> = HashMap::new();

    for row in 0..mx.rows {
        for col in 0..mx.cols {
            if mx.get(row, col) != 0 {
                pivot_col_to_row.insert(col, row);
                break;
            }
        }
    }

    let pivot_cols: Vec<usize> = pivots.iter().map(|p| p.1).collect();

    let mut out = vec![0; assigns.len()];

    for i in (0..assigns.len()).rev() {
        match assigns[i] {
            None => {
                let row = *pivot_col_to_row.get(&i).expect("should be a pivot row");

                let mut res = mx.get(row, mx.cols - 1);

                for col in i + 1..out.len() {
                    res -= out[col] * mx.get(row, col);
                }

                out[i] = res;
            }
            Some(val) => {
                out[i] = val;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day10() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        // assert_eq!(7, part1(input));
        assert_eq!(33, part2(input));
    }

    // #[test]
    // fn test_reduce() {
    //     let mut mx = vec![vec![1, 0, 1], vec![0, 2, 4]];

    //     for row in mx.iter() {
    //         for col in row {
    //             print!("{:3}", col);
    //         }
    //         println!();
    //     }
    //     println!();

    //     // reduce(&mut mx);

    //     for row in mx.iter() {
    //         for col in row {
    //             print!("{:3}", col);
    //         }
    //         println!();
    //     }
    //     println!();

    //     panic!();
    // }

    #[test]
    fn test_reduce() {
        let mut mx = Matrix::from_rows(vec![
            vec![1, 3, 1, 9],
            vec![1, 1, -1, 1],
            vec![3, 11, 5, 35],
        ]);

        reduce(&mut mx);

        let expect =
            Matrix::from_rows(vec![vec![1, 0, -2, -3], vec![0, 1, 1, 4], vec![0, 0, 0, 0]]);

        assert_eq!(mx, expect);

        let end = assign_frees(&mx, vec![None, None, Some(3)]);

        assert_eq!(end, vec![3, 1, 3]);
    }

    #[test]
    fn test_matrix_operations() {
        let mut mx = Matrix::from_rows(vec![
            vec![1, 3, 1, 9],
            vec![1, 1, -1, 1],
            vec![3, 11, 5, 35],
        ]);

        mx.add_row_multiple(1, 0, -1);
        mx.add_row_multiple(2, 0, -3);

        let expect1 = Matrix::from_rows(vec![
            vec![1, 3, 1, 9],
            vec![0, -2, -2, -8],
            vec![0, 2, 2, 8],
        ]);

        assert_eq!(mx, expect1);

        mx.add_row_multiple(2, 1, 1);

        let expect2 = Matrix::from_rows(vec![
            vec![1, 3, 1, 9],
            vec![0, -2, -2, -8],
            vec![0, 0, 0, 0],
        ]);

        assert_eq!(mx, expect2);

        mx.normalize_row(1);

        let expect3 = Matrix::from_rows(vec![vec![1, 3, 1, 9], vec![0, 1, 1, 4], vec![0, 0, 0, 0]]);

        assert_eq!(mx, expect3);

        let frees = get_free_vars(&mx);

        assert_eq!(frees, vec![2]);
    }
}

/*
  1  0  1  1  0  7
  0  0  0  1  1  5
  1  1  0  1  1 12
  1  1  0  0  1  7
  1  0  1  0  1  2
*/
