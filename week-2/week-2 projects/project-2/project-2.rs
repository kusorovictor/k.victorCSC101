fn main() {
    //Initalize a String array that stores the names of all the items
    let items: [String; 5] = [
        String::from("Toshiba"),
        String::from("Mac"),
        String::from("HP"),
        String::from("Dell"),
        String::from("Acer"),
    ];
    //Initalize a String array that stores the prices of all the items respective of their positions
    let prices: [f32; 5] = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    //Initalize a String array that stores the quantities of all the items respective of their positions
    let quantities: [f32; 5] = [2.0, 1.0, 3.0, 3.0, 1.0];

    let mut index = 0; //Initalize a mutable variable that keeps track of the array index

    let mut total_price: f32 = 0.0;

    //loop through the length of the items array
    while index < items.len() {
        let actual_price: f32 = prices[index] * quantities[index];

        println!("Item: {} -> Actual Price: ₦{} ", items[index], actual_price);

        index += 1;

        total_price += actual_price; //calculate the total price
    }

    println!("Total Price: ₦{} ", total_price);

    let average_price = total_price / quantities.len() as f32; //calculate the average price
    println!("Average Price: ₦{} ", average_price);
}
