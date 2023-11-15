use std::convert::TryInto;

fn main() {
    let arr = [0, 1, -1, 2, -2, 3, -3];

    for n in arr {
        let un: Result<u32, _> = n.try_into();

        if un.is_err() {
            println!("{n} cannot be converted to unsigned int");
        } else {
            println!("{n} can be converted to unsigned int");
        }
    }
}
