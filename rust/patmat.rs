use std::convert::TryInto;

fn main() {
    let arr = [0, 1, -1, 2, -2, 3, -3];

    for n in arr {
        let un: Result<u32, _> = n.try_into();

        match un {
            Ok(value) => println!("{value} was converted to unsigned"),
            // underscore indicates we don't care about the value
            Err(_) => println!("Could not convert {n}"),
        }
    }
}
