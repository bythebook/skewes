use skewes::Natural;

fn factorial(n: Natural) -> Natural {
    if n == Natural::from(1) {
        Natural::from(1)
    }
    else {
        let mut acc = Natural::from(1);
        let mut m = n.clone();
        while m > Natural::ZERO {
            acc = &acc * &m;
            m = m.sub(&Natural::from(1)).unwrap();
        }
        acc
    }
}

fn main() {
    let n = Natural::from(100);
    println!("100!: {}", factorial(n));
}