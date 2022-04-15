const LENGTH: usize = 15;

fn main() {
    let mut numbers = [0; LENGTH];

    numbers[0] = 1;
    numbers[1] = 1;

    for i in 2..LENGTH {
        numbers[i] = numbers[i-1] + numbers[i-2];
    }

    for n in numbers.iter() {
        println!("{n:>3}");
    }
}
