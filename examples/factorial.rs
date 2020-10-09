use skewes::Natural;

fn factorial(n: Natural) -> Natural {
    if n == Natural::one() {
        Natural::one()
    }
    else {
        let mut acc = Natural::one();
        let mut m = n.clone();
        while m > Natural::zero() {
            acc = &acc * &m;
            m = m.sub(&Natural::one()).unwrap();
        }
        acc
    }
}

fn main() {
    let n = Natural::from(100);
    println!("100!: {}", factorial(n));
}