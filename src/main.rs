use core::panic;
use std::{collections::HashMap, fs};

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
    let mut joltage = 0;

    for line in input.lines() {
        let digits: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let line_len = digits.len();
        let mut max = 0;

        // This is super wasteful. We should use the logic from part 2 with
        // 2 in place of 12.
        for i in 0..line_len - 1 {
            let tens = 10 * digits[i];
            for j in i + 1..line_len {
                let val = tens + digits[j];

                max = max.max(val);
            }
        }

        joltage += max as i64;
    }

    joltage
}

fn day3_part2(input: &str) -> i64 {
    let mut joltage: i64 = 0;

    for line in input.lines() {
        let digits: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let line_len = digits.len();

        let mut digit_idxs: [usize; 12] = [0; 12];
        for digit_index in 0..12 {
            let start = if digit_index == 0 {
                0
            } else {
                digit_idxs[digit_index - 1] + 1
            };
            let stop = line_len - 11 + digit_index;

            let mut max_ind = start;
            for i in start + 1..stop {
                let max_val = digits[max_ind];
                let val = digits[i];

                if val > max_val {
                    max_ind = i;
                }
            }

            digit_idxs[digit_index] = max_ind;
        }

        let line_joltage = digit_idxs
            .iter()
            .fold(0, |acc, &idx| 10 * acc + digits[idx] as i64);

        joltage += line_joltage;
    }

    joltage
}

fn day4_part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // println!("{:?}", grid);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut rolls = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '@' {
                let mut neighbors = 0;

                if i > 0 && j > 0 {
                    if grid[i - 1][j - 1] == '@' {
                        neighbors += 1;
                    }
                }

                if i > 0 {
                    if grid[i - 1][j] == '@' {
                        neighbors += 1;
                    }
                }

                if i > 0 && j < cols - 1 {
                    if grid[i - 1][j + 1] == '@' {
                        neighbors += 1;
                    }
                }

                if j > 0 {
                    if grid[i][j - 1] == '@' {
                        neighbors += 1;
                    }
                }

                if j < cols - 1 {
                    if grid[i][j + 1] == '@' {
                        neighbors += 1;
                    }
                }

                if i < rows - 1 && j > 0 {
                    if grid[i + 1][j - 1] == '@' {
                        neighbors += 1;
                    }
                }

                if i < rows - 1 {
                    if grid[i + 1][j] == '@' {
                        neighbors += 1;
                    }
                }

                if i < rows - 1 && j < cols - 1 {
                    if grid[i + 1][j + 1] == '@' {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    rolls += 1;
                }
            }
        }
    }
    rolls
}

fn day4_part2(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut rolls: i64 = 0;

    loop {
        let mut marked: Vec<(usize, usize)> = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '@' {
                    let mut neighbors = 0;

                    if i > 0 && j > 0 {
                        if grid[i - 1][j - 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if i > 0 {
                        if grid[i - 1][j] == '@' {
                            neighbors += 1;
                        }
                    }

                    if i > 0 && j < cols - 1 {
                        if grid[i - 1][j + 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if j > 0 {
                        if grid[i][j - 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if j < cols - 1 {
                        if grid[i][j + 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if i < rows - 1 && j > 0 {
                        if grid[i + 1][j - 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if i < rows - 1 {
                        if grid[i + 1][j] == '@' {
                            neighbors += 1;
                        }
                    }

                    if i < rows - 1 && j < cols - 1 {
                        if grid[i + 1][j + 1] == '@' {
                            neighbors += 1;
                        }
                    }

                    if neighbors < 4 {
                        marked.push((i, j));
                    }
                }
            }
        }

        if marked.is_empty() {
            break;
        }

        for (i, j) in &marked {
            grid[*i][*j] = '.';
        }

        rolls += marked.len() as i64;
    }
    rolls
}

fn day5_part1(input: &str) -> i64 {
    let mut getting_ranges = true;

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut available = 0;

    for line in input.lines() {
        if getting_ranges && line.is_empty() {
            getting_ranges = false;
            continue;
        }

        if getting_ranges {
            let [low, hi] = line
                .split("-")
                .map(|e| e.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            ranges.push((low, hi));
        } else {
            let candidate: i64 = line.parse().unwrap();
            if ranges
                .iter()
                .any(|&(lo, hi)| lo <= candidate && candidate <= hi)
            {
                available += 1;
            }
        }
    }

    available
}

fn day5_part2(input: &str) -> i64 {
    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let [low, hi] = line
            .split("-")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        ranges.push((low, hi));
    }

    ranges.sort();

    let mut merged: Vec<(i64, i64)> = Vec::new();

    let mut active = ranges[0];

    for &r in &ranges[1..] {
        if r.0 > active.1 + 1 {
            merged.push(active);
            active = r;
        } else if r.1 > active.1 {
            active = (active.0, r.1);
        }
    }

    merged.push(active);

    merged.iter().map(|r| r.1 - r.0 + 1).sum()
}

fn day6_part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let nums: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    let ops: Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();

    let mut tot = 0;

    for i in 0..ops.len() {
        match ops[i] {
            "+" => {
                let mut col_acc = nums[0][i];
                for row in 1..lines.len() - 1 {
                    col_acc += nums[row][i];
                }
                tot += col_acc;
            }
            "*" => {
                let mut col_acc = nums[0][i];
                for row in 1..lines.len() - 1 {
                    col_acc *= nums[row][i];
                }
                tot += col_acc;
            }
            _ => panic!("xdd"),
        }
    }

    tot
}

fn day6_part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let char_grid: Vec<Vec<char>> = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let op_locs: Vec<usize> = lines[lines.len() - 1]
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c {
            ' ' => None,
            _ => Some(i),
        })
        .collect();

    let mut overall = 0;

    for loc_idx in 0..op_locs.len() {
        let op = lines[lines.len() - 1].chars().collect::<Vec<char>>()[op_locs[loc_idx]];

        let start = op_locs[loc_idx];
        let end = if loc_idx < op_locs.len() - 1 {
            op_locs[loc_idx + 1] - 1
        } else {
            lines[lines.len() - 1].len()
        };

        match op {
            '+' => {
                let mut tot = 0;
                for col in start..end {
                    let mut tot_num: i64 = 0;
                    // let mut coeff = 1;
                    for row in 0..lines.len() - 1 {
                        let c = char_grid[row][col];
                        if c != ' ' {
                            tot_num = 10 * tot_num + c.to_digit(10).unwrap() as i64;
                        }
                        // print!("{}", char_grid[row][col])
                    }

                    tot += tot_num;
                }
                overall += tot;
            }
            '*' => {
                let mut tot = 1;
                for col in start..end {
                    let mut tot_num: i64 = 0;
                    // let mut coeff = 1;
                    for row in 0..lines.len() - 1 {
                        let c = char_grid[row][col];
                        if c != ' ' {
                            tot_num = 10 * tot_num + c.to_digit(10).unwrap() as i64;
                        }
                    }
                    tot *= tot_num;
                }
                overall += tot;
            }
            _ => panic!(),
        }
    }
    overall
}

fn day7_part1(input: &str) -> i64 {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for col in 0..lines[0].len() {
        if lines[0][col] == 'S' {
            lines[1][col] = '|';
            break;
        }
    }

    let mut splits = 0;

    for row in 0..lines.len() - 1 {
        for col in 0..lines[row].len() {
            if lines[row][col] == 'S' || lines[row][col] == '|' {
                if lines[row + 1][col] == '.' {
                    lines[row + 1][col] = '|';
                } else if lines[row + 1][col] == '^' {
                    lines[row + 1][col - 1] = '|';
                    lines[row + 1][col + 1] = '|';
                    splits += 1;
                }
            }
        }
    }

    splits
}

fn day7_part2(input: &str) -> i64 {
    let mut lines: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => 1,
                    '.' => 0,
                    '^' => -1,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    for row in 0..lines.len() - 1 {
        for col in 0..lines[row].len() {
            if lines[row][col] > 0 {
                if lines[row + 1][col] >= 0 {
                    lines[row + 1][col] += lines[row][col];
                } else {
                    lines[row + 1][col - 1] += lines[row][col];
                    lines[row + 1][col + 1] += lines[row][col];
                }
            }
        }
    }

    lines.last().unwrap().iter().sum()
}

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2) + (b[2] - a[2]).powi(2)).sqrt()
}

fn day8_part1(input: &str, connections: usize) -> i64 {
    let coords: Vec<[f64; 3]> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut adj = vec![vec![false; coords.len()]; coords.len()];

    for _ in 0..connections {
        let mut lo: Option<(usize, usize)> = None;

        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                // println!("From {} to {}", i, j);

                if adj[i][j] {
                    continue;
                }
                match lo {
                    None => {
                        lo = Some((i, j));
                    }
                    Some((a, b)) => {
                        if dist(coords[i], coords[j]) < dist(coords[a], coords[b]) {
                            lo = Some((i, j));
                        }
                    }
                }
            }
        }

        adj[lo.unwrap().0][lo.unwrap().1] = true;
        adj[lo.unwrap().1][lo.unwrap().0] = true; // Don't need this really.
    }

    let mut visited = vec![false; coords.len()];

    let mut components: Vec<Vec<usize>> = Vec::new();

    for i in 0..coords.len() {
        if visited[i] {
            continue;
        }

        let mut comp = Vec::new();

        dfs_sub(&adj, &mut visited, i, &mut comp);

        components.push(comp);
    }

    components.sort_by_key(|comp| comp.len());
    components.reverse();

    let mut prod = 1;

    for pl in 0..3 {
        prod *= components[pl].len();
    }

    prod as i64
}

fn day8_part2(input: &str) -> i64 {
    let coords: Vec<[f64; 3]> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut adj = vec![vec![false; coords.len()]; coords.len()];

    loop {
        let mut lo: Option<(usize, usize)> = None;

        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                // println!("From {} to {}", i, j);

                if adj[i][j] {
                    continue;
                }
                match lo {
                    None => {
                        lo = Some((i, j));
                    }
                    Some((a, b)) => {
                        if dist(coords[i], coords[j]) < dist(coords[a], coords[b]) {
                            lo = Some((i, j));
                        }
                    }
                }
            }
        }

        adj[lo.unwrap().0][lo.unwrap().1] = true;
        adj[lo.unwrap().1][lo.unwrap().0] = true; // Don't need this really.

        let mut visited = vec![false; coords.len()];

        let mut comp = Vec::new();

        dfs_sub(&adj, &mut visited, 0, &mut comp);

        if comp.len() == coords.len() {
            return coords[lo.unwrap().0][0] as i64 * coords[lo.unwrap().1][0] as i64;
        }
    }
}

fn dfs_sub(adj: &Vec<Vec<bool>>, visited: &mut Vec<bool>, start: usize, comp: &mut Vec<usize>) {
    visited[start] = true;
    comp.push(start);

    for targ in 0..adj.len() {
        if adj[start][targ] && !visited[targ] {
            dfs_sub(adj, visited, targ, comp);
        }
    }
}

fn day9_part1(input: &str) -> i64 {
    let coords: Vec<[i64; 2]> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let mut max = -1;

    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            max = max.max(
                (coords[i][0] - coords[j][0] + 1).abs() * (coords[i][1] - coords[j][1] + 1).abs(),
            )
        }
    }

    max
}

#[derive(Debug)]
struct MatrixPos{
    row: usize,
    col: usize
}

fn day9_part2(input: &str) -> i64 {
    let coords: Vec<MatrixPos> = input
        .lines()
        .map(|l| {
            let v = l.split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
                MatrixPos{row: v[1], col: v[0]}
        })
        .collect();

    let mut rows: Vec<usize> = coords.iter().map(|c| c.row).collect();
    rows.sort();
    rows.dedup();

    let mut row_orig_to_comp: HashMap<usize, usize> = HashMap::new();
    let mut row_comp_to_orig: HashMap<usize, usize> = HashMap::new();

    let mut comp_row = 0;

    row_orig_to_comp.insert(rows[0], 0);
    row_comp_to_orig.insert(0, rows[0]);

    for i in 1..rows.len() {
        // Don't create gaps.
        if rows[i] == rows[i-1] + 1 {
            comp_row += 1;
        } else {
            comp_row += 2;
        }
        row_orig_to_comp.insert(rows[i], comp_row);
        row_comp_to_orig.insert(comp_row, rows[i]);
    }

    let mut cols: Vec<usize> = coords.iter().map(|c| c.col).collect();
    cols.sort();
    cols.dedup();

    let mut col_orig_to_comp: HashMap<usize, usize> = HashMap::new();
    let mut col_comp_to_orig: HashMap<usize, usize> = HashMap::new();

    let mut comp_col = 0;

    col_orig_to_comp.insert(cols[0], 0);
    col_comp_to_orig.insert(0, cols[0]);

    for i in 1..cols.len() {
        // Don't create gaps.
        if cols[i] == cols[i-1] + 1 {
            comp_col += 1;
        } else {
            comp_col += 2;
        }
        col_orig_to_comp.insert(cols[i], comp_col);
        col_comp_to_orig.insert(comp_col, cols[i]);
    }

    let coords_compressed: Vec<MatrixPos> = coords.iter().map(|c| MatrixPos{
        row: *row_orig_to_comp.get(&c.row).unwrap(),
        col: *col_orig_to_comp.get(&c.col).unwrap(),
    }).collect();


    let mut drawing = vec![vec!['.'; cols.last().unwrap() + 1]; rows.last().unwrap()+ 1];

    for c in coords_compressed.iter() {
        drawing[c.row][c.col] = '#';
    }

    for i in 0..coords_compressed.len() {
        let next_ind = if i < coords_compressed.len() - 1 { i + 1 } else { 0 };

        let start = &coords_compressed[i];
        let end = &coords_compressed[next_ind];

        if start.row == end.row {
            for col in start.col.min(end.col)+1..start.col.max(end.col) {
                drawing[start.row][col] = 'X';
            }
        } else {
            for row in start.row.min(end.row)+1..start.row.max(end.row) {
                drawing[row][start.col] = 'X';
            }  
        }
    }

    let mut top_corn = 0;

    for col in 0..=comp_col {
        if drawing[0][col] != '.' {
            top_corn = col;
            break;
        }
    }

    let mut stack: Vec<MatrixPos> = vec![MatrixPos{row: 1, col: top_corn+1}];

    while let Some(pos) = stack.pop() {
        drawing[pos.row][pos.col] = 'X';

        // One row up.
        if pos.row > 0 && drawing[pos.row - 1][pos.col] == '.' {
            stack.push(MatrixPos { row: pos.row-1, col: pos.col });
        }

        // One row down.
        if pos.row < comp_row && drawing[pos.row+1][pos.col] == '.' {
            stack.push(MatrixPos { row: pos.row+1, col: pos.col });
        }

        // One col left.
        if pos.col > 0 && drawing[pos.row][pos.col - 1] == '.' {
            stack.push(MatrixPos { row: pos.row, col: pos.col -1 });
        }

        // One col right.
        if pos.col < comp_col && drawing[pos.row][pos.col + 1] == '.' {
            stack.push(MatrixPos { row: pos.row, col: pos.col +1 });
        }
    }

    let mut max = 0;

    for i in 0..coords.len() {
        'xdd: for j in i + 1..coords.len() {
            let height = coords[i].row.max(coords[j].row) - coords[i].row.min(coords[j].row) + 1;
            let width = coords[i].col.max(coords[j].col) - coords[i].col.min(coords[j].col) + 1;

            for row in coords_compressed[i].row.min(coords_compressed[j].row)..=coords_compressed[i].row.max(coords_compressed[j].row) {
                for col in coords_compressed[i].col.min(coords_compressed[j].col)..=coords_compressed[i].col.max(coords_compressed[j].col) {
                    if drawing[row][col] == '.' {
                        continue 'xdd;
                    }
                }
            }

            max = max.max(height * width)
        }
    }

    max as i64
}

fn main() {
    // let day1_input = fs::read_to_string("input1").unwrap();
    // println!("D1P1: {}", day1_part1(&day1_input));
    // println!("D1P2: {}", day1_part2(&day1_input));

    // let day2_input = fs::read_to_string("input2").unwrap();
    // println!("D2P1: {}", day2_part1(&day2_input));
    // println!("D2P2: {}", day2_part2(&day2_input));

    // let day3_input = fs::read_to_string("input3").unwrap();
    // println!("D3P1: {}", day3_part1(&day3_input));
    // println!("D3P2: {}", day3_part2(&day3_input));

    // let day4_input = fs::read_to_string("input4").unwrap();
    // println!("D4P1: {}", day4_part1(&day4_input));
    // println!("D4P2: {}", day4_part2(&day4_input));

    // let day5_input = fs::read_to_string("input5").unwrap();
    // println!("D5P1: {}", day5_part1(&day5_input));
    // println!("D5P2: {}", day5_part2(&day5_input));

    // let day6_input = fs::read_to_string("input6").unwrap();
    // println!("D6P1: {}", day6_part1(&day6_input));
    // println!("D6P2: {}", day6_part2(&day6_input));

    // let day7_input = fs::read_to_string("input7").unwrap();
    // println!("D7P1: {}", day7_part1(&day7_input));
    // println!("D7P2: {}", day7_part2(&day7_input));

    // let day8_input = fs::read_to_string("input8").unwrap();
    // // println!("D8P1: {}", day8_part1(&day8_input, 1000));
    // println!("D8P2: {}", day8_part2(&day8_input));

    let day9_input = fs::read_to_string("input9").unwrap();
    // println!("D8P1: {}", day8_part1(&day8_input, 1000));
    println!("D9P1: {}", day9_part1(&day9_input));
    println!("D9P2: {}", day9_part2(&day9_input));
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

    #[test]
    fn test_day4() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(13, day4_part1(input));
        assert_eq!(43, day4_part2(input));
    }

    #[test]
    fn test_day5() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(3, day5_part1(input));
        assert_eq!(14, day5_part2(input));
    }

    #[test]
    fn test_day6() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(4277556, day6_part1(input));
        assert_eq!(3263827, day6_part2(input));
    }

    #[test]
    fn test_day7() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(21, day7_part1(input));
        assert_eq!(40, day7_part2(input));
    }

    #[test]
    fn test_day8() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(40, day8_part1(input, 10));
        assert_eq!(25272, day8_part2(input));
    }

    #[test]
    fn test_day9() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(50, day9_part1(input));
        assert_eq!(24, day9_part2(input));
    }
}
