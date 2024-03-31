pub fn saturate_sub(v: usize, a: usize, min: usize) -> usize {
    if v <= min + a {
        min
    } else {
        v - a
    }
}

pub fn saturate_add(v: usize, a: usize, max: usize) -> usize {
    if v >= max - a {
        max
    } else {
        v + a
    }
}
