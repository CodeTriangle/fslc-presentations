use std::ops::Add;

// Struct definition
struct Coords {
    x: i32,
    y: i32,
}

// Struct implementation
impl Add<Self> for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let c = Coords { x: 5, y: 6 };
    let d = Coords { x: 7, y: 1 };

    // uses our own custom implementation
    let e = c + d;

    println!("({}, {})", e.x, e.y);
}
