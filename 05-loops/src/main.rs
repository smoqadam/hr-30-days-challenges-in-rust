fn main() {
    let T: i32 = read_line().trim().parse().unwrap();
    for i in 1..11 {
        println!("{} x {} = {}", T, i, T * i);
    }
}



fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
    return input;
}