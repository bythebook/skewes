use skewes::Natural;

fn main() {
    let bound = Natural::from_string("100000000");
    let mut n = Natural::from(1);
    let mut acc = Natural::from(0);

    while n < bound {
        acc.add_mut(&n);
        n.add_mut(&Natural::from(1));
    }
}