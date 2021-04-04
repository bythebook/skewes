use crate::Natural;

pub (in crate::algorithms) fn shl(p: &Natural, n: usize) -> Natural {
    let mut q = p.clone();
    shl_mut(&mut q, n);
    q
}

pub (in crate::algorithms) fn shl_mut(p: &mut Natural, n: usize) {
    for _ in 1..(n+1) { // n times
        p.digits.insert(0, 0); // Insert a zero in the first position (least significant digit)
    }
}

pub (in crate::algorithms) fn shl_mut_vec(v: &mut Vec<u64>, n: usize) {
    for _ in 1..=n {
        v.insert(0, 0);
    }
}