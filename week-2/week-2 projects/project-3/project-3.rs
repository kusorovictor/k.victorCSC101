fn main() {
    let intial_value: f32 = 510_000.0; //Initial value
    let depreciation_rate: f32 = 5.0; //Rate of depreciation
    let time: f32 = 3.0; //No. of years

    let depreciated_value = intial_value * (1.0 - (depreciation_rate / 100.0)).powf(time);

    println!(
        "The value of the TV after {} years is: ₦{} ",
        time, depreciated_value
    );
}
