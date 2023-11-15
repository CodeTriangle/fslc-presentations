fn main() {
    let arr: [i32; 4] = [0, 1, 2, 3];

    // does not exist in array
    let y = arr.get(7);
    println!("{:?}", y);

    // exists in array
    let x = arr.get(2);
    println!("{:?}", x);

    // on a `Some` type, we can use `.unwrap()` to get the value inside
    println!("{}", x.unwrap());
}
