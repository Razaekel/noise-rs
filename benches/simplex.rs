extern crate criterion;
extern crate noise;

use criterion::*;
use noise::{NoiseFn, Simplex};
use rand::Rng;

criterion_group!(simplex, bench_simplex);
// criterion_group!(
//     simplex_64x64,
//     bench_simplex2_64x64,
//     bench_simplex3_64x64,
//     bench_simplex4_64x64
// );
criterion_main!(simplex);

fn bench_simplex(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simplex");

    // let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    // group.plot_config(plot_config);

    let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());

    let simplex = Simplex::default();

    for step in 0..10 {
        let size = 1 << step;

        group.throughput(Throughput::Elements(size as u64));

        // Generate input arrays
        let input_1 = criterion::black_box((0..size).map(|_| rng.gen()).collect::<Vec<f64>>());
        let input_2 = criterion::black_box((0..size).map(|_| rng.gen()).collect::<Vec<f64>>());

        // prefill output vector with random values
        let mut output = criterion::black_box((0..size).map(|_| rng.gen()).collect::<Vec<f64>>());

        group.bench_with_input(BenchmarkId::new("2D", size), &size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    output[i] = criterion::black_box(simplex.get([input_1[i], input_2[i]]));
                }
            })
        });
    }

    group.finish();
}
