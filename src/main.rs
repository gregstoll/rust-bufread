use std::io;

use rust_bufread::*;

fn main() -> io::Result<()> {
    println!("{}", read_unbuffered_one_character_at_a_time()?);
    println!("{}", read_buffer_whole_string_into_memory()?);
    println!("{}", read_buffered_allocate_string_every_time()?);
    println!("{}", read_buffered_reuse_string()?);
    Ok(())
}
