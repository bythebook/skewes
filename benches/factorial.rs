use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skewes::Natural;

fn factorial(n: Natural) -> Natural {
    if n == Natural::one() {
        Natural::one()
    }
    else {
        let mut acc = Natural::one();
        let mut m = n.clone();
        while m > Natural::zero() {
            acc = acc.mul(&m);
            m = m.sub(&Natural::one()).unwrap();
        }
        acc
    }
}

fn fact_benchmark(c: &mut Criterion) {
    c.bench_function("20!", |b| b.iter(|| factorial(black_box(Natural::from(20)))));
}

criterion_group!(benches, fact_benchmark);
criterion_main!(benches);