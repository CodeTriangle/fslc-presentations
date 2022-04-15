struct Coords {
    x: i32,
    y: i32,
}

// changing fields on a mutable reference
fn invert(c: &Coords) -> Coords {
    Coords { x: -c.x, y: -c.y }
}

fn main() {
    let c = Coords { x: 5, y: 6 };

    let d: Coords = invert(&c);

    println!("{}, {}", d.x, d.y); // "-5, -6"

    // SUCCEEDS; prints negative of coords.
    println!("{}, {}", c.x, c.y); // "5, 6"
}
