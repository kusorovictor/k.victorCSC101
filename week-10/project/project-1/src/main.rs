fn main() {
    println!("Total Price: {}", calculate_total())
}

//Defined the laptop class with the brand price and quantity
struct Laptop {
    brand: String,
    price: i32,
    quantity: i16,
}

impl Laptop {
    //Method to calaculate the price of 3 laptops
    fn total_price(&self) -> i32 {
        self.price * 3
    }
}

// Method to set the values of a laptop object
fn set_values(brand: String, price: i32, quantity: i16) -> Laptop {
    Laptop {
        brand,
        price,
        quantity,
    }
}

//Calculate the total_price of all the laptops
fn calculate_total() -> i32 {
    let laptops = [
        set_values("HP".to_string(), 650_000, 10),
        set_values("IBM".to_string(), 755_000, 6),
        set_values("Toshiba".to_string(), 550_000, 10),
        set_values("Dell".to_string(), 850000, 4),
    ];

    let mut total_price = 0;

    for laptop in laptops {
        total_price += laptop.total_price();
        println!(
            "Brand -> {}, Cost of 3 -> {}, Remaining Quantity -> {}",
            laptop.brand,
            laptop.total_price(),
            (laptop.quantity - 3)
        );
    }

    total_price
}
