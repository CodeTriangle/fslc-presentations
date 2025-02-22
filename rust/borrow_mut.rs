struct Coords { // in rust we use "struct" not "class"
    x: i32,
    y: i32,
}

// modify c in-place
fn invert(c: &mut Coords) {
    c.x = -c.x;
    c.y = -c.y;
}

fn main() {
    let mut c = Coords { x: 5, y: 6 };

    println!("{}, {}", c.x, c.y); // "5, 6"

    invert(&mut c);

    println!("{}, {}", c.x, c.y); // "-5, -6"
}
