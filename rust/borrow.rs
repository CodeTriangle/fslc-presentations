struct Coords {
    x: i32,
    y: i32,
}

// copy c but make both options negative
// note the ampersand -- that designates a reference
fn invert(c: &Coords) -> Coords {
    Coords { x: -c.x, y: -c.y }
}

fn main() {
    let c = Coords { x: 5, y: 6 };

    let d: Coords = invert(&c);

    println!("{}, {}", d.x, d.y); // "-5, -6"

    println!("{}, {}", c.x, c.y); // "5, 6"
}
