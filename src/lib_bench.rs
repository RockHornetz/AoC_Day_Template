use criterion::{criterion_group, criterion_main, Criterion};
use {{project-name}}::{ {{class_name}} as AocDay, INPUT_STR}; // Import functions from your library

pub fn bench_part_01_final(c: &mut Criterion) {
    let mut aoc_day = AocDay::new(INPUT_STR).unwrap();
    c.bench_function("{{project_name}} part_01_final", |b| b.iter(|| aoc_day.process_part_01()));
}

pub fn bench_part_02_final(c: &mut Criterion) {
    let mut aoc_day = AocDay::new(INPUT_STR).unwrap();
    c.bench_function("{{project_name}} part_02_final", |b| b.iter(|| aoc_day.process_part_02()));
}

criterion_group!( {{project-name}}, bench_part_01_final, bench_part_02_final);
criterion_main!( {{project-name}} );