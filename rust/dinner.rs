enum Size {
    Large,
    Medium,
    Small,
}

enum Sandwich {
    Chicken, // only one chicken sandwich on menu
    Burger(i32), // argument = number of patties
}

enum Order {
    Combo {
        sandwich: Sandwich,
        size: Size,
    },
    SandwichOnly(Sandwich),
}
