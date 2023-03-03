use rust_bufread::*;

fn main() {
    println!("{}", read_whole_string_into_memory());
    println!("{}", read_buffered_allocate_string_every_time());
}
