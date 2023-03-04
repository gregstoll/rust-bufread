use std::{fs::File, io::{Read, BufReader, BufRead, self}};

const FILENAME: &str = "word_frequency.txt";

pub fn read_unbuffered_one_character_at_a_time() -> io::Result<u64> {
    let mut f = File::open(FILENAME)?;
    let len = f.metadata().expect("Failed to get file metadata").len() as usize;
    let mut v: Vec<u8> = Vec::new();
    v.resize(len, 0u8);
    for index in 0..len {
        f.read_exact(&mut v[index..(index+1)])?;
    }
    let s = String::from_utf8(v).expect("file is not UTF-8?");
    let mut total = 0u64;
    for line in s.lines() {
        total += get_count_from_line(line);
    }
    Ok(total)
}

pub fn read_buffered_allocate_string_every_time() -> io::Result<u64> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);

    let mut total = 0u64;
    for line in reader.lines() {
        let s = line?;
        total += get_count_from_line(&s);
    }
    Ok(total)
}

pub fn read_buffered_reuse_string() -> io::Result<u64> {
    let file = File::open(FILENAME)?;
    let mut reader = BufReader::new(file);

    let mut string = String::new();
    let mut total = 0u64;
    while reader.read_line(&mut string).unwrap() > 0 {
        total += get_count_from_line(&string);
        string.clear();
    }
    Ok(total)
}

pub fn read_buffer_whole_string_into_memory() -> io::Result<u64> {
    let mut file = File::open(FILENAME)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut total = 0u64;
    for line in s.lines() {
        total += get_count_from_line(line);
    }
    Ok(total)
}

fn get_count_from_line(s: &str) -> u64 {
    if s.is_empty() {
        return 0;
    }
    let mut parts = s.split_ascii_whitespace();
    let _ = parts.next();
    parts.next().unwrap().parse::<u64>().unwrap()
}