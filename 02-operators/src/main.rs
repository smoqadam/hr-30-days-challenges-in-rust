fn main() {
    let mut meal_cost = String::new();
    match std::io::stdin().read_line(&mut meal_cost) {
        Ok(_)=>{},
        Err(why)=>println!("{}", why),
    };
    let meal_cost: f32 = meal_cost.trim().parse().unwrap();

    let mut tip_percent = String::new();
    match std::io::stdin().read_line(&mut tip_percent) {
        Ok(_) => {},
        Err(why) => println!("{}", why),
    };
    let tip_percent: f32 = tip_percent.trim().parse().unwrap();


    let mut tax_percent = String::new();
    match std::io::stdin().read_line(&mut tax_percent) {
        Ok(_) => {},
        Err(why) => println!("{}", why),
    };
    let tax_percent: f32 = tax_percent.trim().parse().unwrap();

    let tip = meal_cost * (tip_percent / 100.0) ;
    let tax = meal_cost * (tax_percent / 100.0);
    let total_cost = meal_cost + tax + tip ;
    println!("The total meal cost is {} dollars.", total_cost.round());
}
