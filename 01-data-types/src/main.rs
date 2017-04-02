use std::io;

fn main() {
    let i: i32 = 4;
    let d: f32 = 4.0;
    let s: &str = "HackerRank ";

    let mut i_str = String::new();
    match io::stdin().read_line(&mut i_str) {
        Ok(_) => {}
        Err(why) => println!("{}", why),
    }
    let i2: i32 = i_str.trim().parse().unwrap();

    let mut d_str = String::new();
    match io::stdin().read_line(&mut d_str) {
        Ok(_) => {}
        Err(why) => println!("{}", why),
    }
    let d2: f32 = d_str.trim().parse().unwrap();

    let mut s2 = String::new();
    match io::stdin().read_line(&mut s2) {
        Ok(_) => {}
        Err(why) => println!("{}", why),
    }
    println!("{}", i + i2);
    println!("{:.1}", d + d2);
    println!("{}{}", s, s2);
}