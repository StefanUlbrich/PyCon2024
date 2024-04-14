#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::izip;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray_npy::read_npy;

struct Input {
    data: Array2<f64>,
}

fn data_fixture() -> Input {
    todo!()
}

fn test(input: Input) {
    todo!();
}

fn one_bench(c: &mut Criterion) {
    c.bench_function("one_bench", |b| b.iter(|| test(data_fixture())));
}

criterion_group!(benches, one_bench);

criterion_main!(benches);
