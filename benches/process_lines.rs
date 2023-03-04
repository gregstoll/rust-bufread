use criterion::{criterion_group, criterion_main, Criterion};
use rust_bufread::*;

fn bench_unbuffered_one_character_at_a_time(c: &mut Criterion) {
    c.bench_function("unbuffered_one_character_at_a_time", |b| b.iter(|| read_unbuffered_one_character_at_a_time()));
}

fn bench_buffer_whole_string_into_memory(c: &mut Criterion) {
    c.bench_function("buffer_whole_string_into_memory", |b| b.iter(|| read_buffer_whole_string_into_memory()));
}

fn bench_buffered_allocate_string_every_time(c: &mut Criterion) {
    c.bench_function("buffered_allocate_string_every_time", |b| b.iter(|| read_buffered_allocate_string_every_time()));
}

fn bench_buffered_reuse_string(c: &mut Criterion) {
    c.bench_function("buffered_reuse_string", |b| b.iter(|| read_buffered_reuse_string()));
}

criterion_group!(
    name = benches;
    // Set the sample size lower than the default of 100 because the unbuffered version
    // is extremely slow.
    config = Criterion::default().sample_size(10);
    targets = bench_unbuffered_one_character_at_a_time,
        bench_buffer_whole_string_into_memory,
        bench_buffered_allocate_string_every_time,
        bench_buffered_reuse_string);
criterion_main!(benches);