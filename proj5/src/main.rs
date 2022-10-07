fn main() {
    println!("Hello, world!");
    let mut number = (400, 200, 300);
    let (new_number, data, y) = number;
    println!("Number, {data}");
    number = (233, 322, 123);
    let (new_number, data, y) = number;
    println!("Number, {data}");
}
