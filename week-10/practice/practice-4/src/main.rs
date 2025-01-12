fn main() {
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];

    print_vector(v); // this line gives error
    println!("{}", v[0]);
}

fn print_vector(x: Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
