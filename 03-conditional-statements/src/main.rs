fn main() {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(why) => println!("{}", why),
    };
    let input_int: i32 = input.trim().parse().unwrap();
    match input_int {
        x if (x % 2 != 0) => println!("Weird"),
        2...5 => println!("Not Weird"),
        6...20 => println!("Weird"),
        _ =>  println!("Not Weird"),
    }

}
