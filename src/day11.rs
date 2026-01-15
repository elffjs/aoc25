use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (device, outputs_raw) = line.split_once(':').unwrap();

        graph.insert(
            device.to_string(),
            outputs_raw
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        );
    }

    count_paths(&graph, "you".to_string(), "out".to_string()) as i64
} 

fn inner(start: String, end: String, graph: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(exist) = cache.get(&start) {
        *exist 
    } else if start == end{
        1
    } else if let Some(desc) = graph.get(&start){
        let mut tot = 0;
        for out in desc {
            tot += inner(out.clone(), end.clone(), graph, cache);
        }
        cache.insert(start, tot);
        tot
    } else {
        0
    }
}

fn count_paths(graph: &HashMap<String, Vec<String>>, start: String, end: String) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();

    inner(start, end, graph, &mut cache)
}

pub fn part2(input: &str) -> i64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (device, outputs_raw) = line.split_once(':').unwrap();

        graph.insert(
            device.to_string(),
            outputs_raw
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        );
    }

    (count_paths(&graph, "svr".to_string(), "dac".to_string())
        * count_paths(&graph, "dac".to_string(), "fft".to_string())
        * count_paths(&graph, "fft".to_string(), "out".to_string())
        + count_paths(&graph, "svr".to_string(), "fft".to_string())
            * count_paths(&graph, "fft".to_string(), "dac".to_string())
            * count_paths(&graph, "dac".to_string(), "out".to_string())) as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_was() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let input2 = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        assert_eq!(5, part1(input));
        assert_eq!(2, part2(input2));
    }
}
