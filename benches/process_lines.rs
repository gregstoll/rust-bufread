use criterion::{criterion_group, criterion_main, Criterion};
use rust_bufread::*;

fn bench_whole_string_into_memory(c: &mut Criterion) {
    c.bench_function("whole_string_into_memory", |b| b.iter(|| read_whole_string_into_memory()));
}

fn bench_buffered_allocate_string_every_time(c: &mut Criterion) {
    c.bench_function("buffered_allocate_string_every_time", |b| b.iter(|| read_buffered_allocate_string_every_time()));
}

fn bench_buffered_reuse_string(c: &mut Criterion) {
    c.bench_function("buffered_reuse_string", |b| b.iter(|| read_buffered_reuse_string()));
}

criterion_group!(benches, bench_whole_string_into_memory, bench_buffered_allocate_string_every_time, bench_buffered_reuse_string);
criterion_main!(benches);