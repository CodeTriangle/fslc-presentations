fn main() {
    let mut numbers = Box::new([0; 16]); // array of length 16

    numbers[0] = 1;
    numbers[1] = 1;

    for i in 2..numbers.len() { // rust has a range type with syntax x..y
        numbers[i] = numbers[i-1] + numbers[i-2];
    }

    for n in numbers.iter() {
        println!("{n:>3}"); // println is a macro, hence the !
    }
}
