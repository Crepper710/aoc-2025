use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.trim_end().lines().map(|line| {
        let (key, values) = line.split_once(": ").unwrap();
        (key, values.split(' ').collect())
    }).collect()
}

fn reverse_connections<'a>(connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&'a str, HashSet<&'a str>> {
    connections.iter().fold(HashMap::new(), |mut acc, (key, values)| {
        values.iter().for_each(|value| {acc.entry(value).or_insert(HashSet::new()).insert(key);});
        acc
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let connections = parse(input);
    let reversed_connections = reverse_connections(&connections);
    let relevant_connections = get_notes_from("you", &connections);
    paths("you", "out", &filter_connections(relevant_connections, &reversed_connections))
}

fn get_notes_from<'a>(from: &'a str, connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
    let mut found = HashSet::new();
    found.insert(from);
    let mut queue = VecDeque::new();
    queue.push_back(from);
    while let Some(current) = queue.pop_front() {
        if let Some(next) = connections.get(current) {
            for next in next.iter() {
                if found.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }
    found
}

fn get_notes_from_to<'a>(from: &'a str, to: &'a str, connections: &'a HashMap<&'a str, HashSet<&'a str>>, reversed_connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
    let from_notes = get_notes_from(from, connections);
    let to_notes = get_notes_from(to, reversed_connections);
    from_notes.intersection(&to_notes).cloned().collect()
}

fn filter_connections<'a>(relevant_connections: HashSet<&'a str>, connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashMap<&'a str, HashSet<&'a str>> {
    connections.iter().filter_map(|(key, value)| if relevant_connections.contains(*key) {Some((*key, value.clone()))} else {None}).collect()
}

fn paths(from: &str, to: &str, reversed_connections: &HashMap<&str, HashSet<&str>>) -> Option<u64> {
    let mut paths_to_out = HashMap::new();
    paths_to_out.insert(to, 1);
    let mut queue = VecDeque::new();
    queue.push_back(to);
    while let Some(current) = queue.pop_front() {
        if let Some(next) = reversed_connections.get(current) {
            for next in next.iter() {
                *paths_to_out.entry(next).or_insert(0) += 1;
                queue.push_back(next);
            }
        }
    }
    paths_to_out.get(from).copied()
}

pub fn part_two(input: &str) -> Option<u64> {
    let connections = parse(input);
    let reversed_connections = reverse_connections(&connections);

    let svr_fft_notes = get_notes_from_to("svr", "fft", &connections, &reversed_connections);
    let fft_dac_notes = get_notes_from_to("fft", "dac", &connections, &reversed_connections);
    let dac_out_notes = get_notes_from_to("dac", "out", &connections, &reversed_connections);

    let svr_fft_paths = paths("svr", "fft", &filter_connections(svr_fft_notes, &reversed_connections))?;
    let fft_dac_paths = paths("fft", "dac", &filter_connections(fft_dac_notes, &reversed_connections))?;
    let dac_out_paths = paths("dac", "out", &filter_connections(dac_out_notes, &reversed_connections))?;

    Some(svr_fft_paths * fft_dac_paths * dac_out_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
