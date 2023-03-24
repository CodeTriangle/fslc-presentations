fn main() {
    let ary = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    ary.iter()
        .map(|num| num * 2 + 1)
        .filter(|odd| *odd < 7)
        .for_each(|num| print!("{3>:num} "))
}
