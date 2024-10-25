fn main() {
    //Initalize the principal, rate, time
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 10.0;
    let time: f64 = 5.0;

    let amount: f64 = principal * (1.0 + (rate / 100.0)).powf(time); //Calculate the amount
    println!("Amount is: ₦{}", amount);

    let interest: f64 = amount - principal; //Calculate the interest
    println!("Simple Interest is: ₦{}", interest);
}
