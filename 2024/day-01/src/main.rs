use std::{
    collections::HashMap,
    env::current_dir,
    fs::File,
    io::{self, BufRead},
};

fn parse_lists(path: &str) -> (Vec<u32>, Vec<u32>, HashMap<u32, u32>) {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();
    let mut similarities: HashMap<u32, u32> = HashMap::new();

    let file =
        File::open(path).unwrap_or_else(|_| panic!("Unable to read file at path ({}).", path));
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let current_line = line.unwrap();
        let values: Vec<u32> = current_line
            .split("   ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        list_1.push(values[0]);
        list_2.push(values[1]);
        similarities.insert(values[0], 0);
    }

    list_1.sort();
    list_2.sort();

    (list_1, list_2, similarities)
}

fn calculate_distance(list_1: &[u32], list_2: &[u32]) -> u32 {
    let mut distance = 0;

    for (index, _location) in list_1.iter().enumerate() {
        let difference = list_1[index].abs_diff(list_2[index]);
        distance += difference;
    }

    distance
}

fn calculate_similarity(mut similarities: HashMap<u32, u32>, list: &[u32]) -> u32 {
    let mut similarity = 0;

    for location in list {
        if let Some(location_match) = similarities.get_mut(location) {
            *location_match += 1;
        }
    }

    for (k, v) in similarities.drain() {
        similarity += k * v
    }

    similarity
}

fn main() {
    let path = format!("{}/locations.txt", current_dir().unwrap().display());
    let (list_1, list_2, similarities) = parse_lists(&path);

    println!(
        "The total distance observed between the two lists is: {}.",
        calculate_distance(&list_1, &list_2)
    );

    println!(
        "The observed similarity between the two lists is: {}.",
        calculate_similarity(similarities, &list_2)
    );
}
