use std::{fs::File, io::{Read, BufReader, BufRead}};

const FILENAME: &str = "word_frequency.txt";

fn get_count_from_line(s: &str) -> u64 {
    let mut parts = s.split_ascii_whitespace();
    let _ = parts.next();
    parts.next().unwrap().parse::<u64>().unwrap()
}

pub fn read_unbuffered_one_character_at_a_time() -> u64 {
    let mut f = File::open(FILENAME).expect("Failed to open file!");
    let mut current_line = Vec::<u8>::with_capacity(100);
    let mut index = 0;
    //TODO
    /*while f.read_exact(&mut current_line[index]) {
    }*/
    return 0u64;
}

pub fn read_whole_string_into_memory() -> u64 {
    let mut file = File::open(FILENAME).expect("Failed to open file!");
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let mut total = 0u64;
    for line in s.lines() {
        total += get_count_from_line(line);
    }
    total
}

pub fn read_buffered_allocate_string_every_time() -> u64 {
    let file = File::open(FILENAME).expect("Failed to open file!");
    let reader = BufReader::new(file);

    let mut total = 0u64;
    for line in reader.lines() {
        let s = line.unwrap();
        total += get_count_from_line(&s);
    }
    total
}