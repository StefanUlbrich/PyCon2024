use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::izip;
use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray_npy::read_npy;

struct Input {
    data: Array2<f64>,
    responsibilities: Array2<f64>,
}

fn data_fixture() -> Input {
    // Note: we need to explicitly state the type here ..
    let data: Array2<f64> = read_npy("data/data.npy").unwrap();
    let responsibilities: Array2<f64> =
        read_npy("data/responsibilities.npy").unwrap();
    let responsibilities = responsibilities.t().to_owned();

    Input {
        data,
        responsibilities,
    }
}

fn c_style(input: Input) {
    let Input {
        data,
        responsibilities,
    } = input;

    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    for i in 0..k {
        let x = adjusted.slice(s![.., i, ..]);
        let y = &x * &responsibilities.slice(s![.., i, NewAxis]);

        covs.slice_mut(s![i, .., ..]).assign(&x.t().dot(&y));
    }

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();
}

fn iterators(input: Input) {
    let Input {
        data,
        responsibilities,
    } = input;

    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    for (x, mut c, r) in izip!(
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1))
    ) {
        let y = &x * &r.slice(s![.., NewAxis]);
        c += &x.t().dot(&y);
    }

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();
}

fn functional(input: Input) {
    let Input {
        data,
        responsibilities,
    } = input;

    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    izip!(
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1))
    )
    .for_each(|(x, mut cov, resp)| {
        let y = &x * &resp.slice(s![.., NewAxis]);
        cov += &x.t().dot(&y);
    });

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();
}

fn parallel(input: Input) {
    let Input {
        data,
        responsibilities,
    } = input;

    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    let mut covs = Array3::<f64>::zeros((k, d, d));

    (
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1)),
    )
        .into_par_iter()
        .for_each(|(x, mut cov, resp)| {
            let y = &x * &resp.slice(s![.., NewAxis]);
            cov += &x.t().dot(&y);
        });

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();
}

fn parallel_bench(c: &mut Criterion) {
    c.bench_function("parallel", |b| b.iter(|| parallel(data_fixture())));
}

fn functional_bench(c: &mut Criterion) {
    c.bench_function("functional", |b| b.iter(|| functional(data_fixture())));
}

fn iterators_bench(c: &mut Criterion) {
    c.bench_function("iterators", |b| b.iter(|| iterators(data_fixture())));
}

fn c_style_bench(c: &mut Criterion) {
    c.bench_function("c_style", |b| b.iter(|| c_style(data_fixture())));
}

criterion_group!(
    benches,
    c_style_bench,
    iterators_bench,
    functional_bench,
    parallel_bench
);
criterion_main!(benches);
