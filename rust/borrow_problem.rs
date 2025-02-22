struct Coords { // in rust we use "struct" not "class"
    x: i32,
    y: i32,
}

// copy c but make both options negative
fn invert(c: Coords) -> Coords {
    Coords { x: -c.x, y: -c.y }
}

fn main() {
    let c = Coords { x: 5, y: 6 };

    let d: Coords = invert(c);

    println!("{}, {}", d.x, d.y); // "-5, -6"

    // This line will error. Why?
    println!("{}, {}", c.x, c.y); // "5, 6"
}
