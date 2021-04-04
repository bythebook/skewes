use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skewes::Natural;

fn factorial(n: Natural) -> Natural {
    if n == Natural::from(1) {
        Natural::from(1)
    }
    else {
        let mut acc = Natural::from(1);
        let mut m = n.clone();
        while m > Natural::ZERO {
            acc = acc.mul(&m);
            m = m.sub(&Natural::from(1)).unwrap();
        }
        acc
    }
}

fn fact_benchmark(c: &mut Criterion) {
    c.bench_function("20!", |b| b.iter(|| factorial(black_box(Natural::from(20)))));
}

criterion_group!(benches, fact_benchmark);
criterion_main!(benches);