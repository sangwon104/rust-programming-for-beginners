// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Apple,
    Banana,
}

struct Drink {
    flavor: Flavor,
    fluid_ounce: f64,
}

fn print_drink_info (drink: Drink) {
    println!("fluid ounce : {:?}", drink.fluid_ounce);
    match drink.flavor {
        Flavor::Orange => println!("flavor : orange!"),
        Flavor::Apple => println!("flavor : apple!"),
        Flavor::Banana => println!("flavor : banana")
    }
}

fn main() {
    let drink: Drink = Drink {
        flavor: Flavor::Orange,
        fluid_ounce: 2.99
    };
    print_drink_info(drink);
}
