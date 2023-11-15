fn main() {
    std::iter::repeat(1) // iterator that returns 1 infinitely
        .take(50) // take the first 60 elements
        .enumerate() // return pairs of (index, value)
        .map(|(i, val)| i + val) // map to a sum
        .filter(|i| *i % 5 == 4) // skip all elements that don't mod to 4
        .map(|i| i * 2) // multiply all elements by two
        .for_each(|num| print!("{num:3} ")) // print each
}
