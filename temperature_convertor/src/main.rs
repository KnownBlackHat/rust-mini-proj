fn main () {
    println!("Convert Temperature Units!");
    let mut unit = String::new();
    println!("Choose K [Kelvin] or C [Degree]");
    std::io::stdin().read_line(&mut unit).expect("Failed to read line");

    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("Failed to readline");
    let temp: i32 = temp.trim().parse().expect("Failed to parse temp in i32");

    match unit.trim() {
        "K"|"k" => convert_celsius(temp),
        "C"|"c" => convert_kelvin(temp),
        _ => println!("Unexpected Input")
    }
}

fn convert_celsius(kelvin: i32) {
    println!("{} C", kelvin - 273 );
}

fn convert_kelvin(celsius: i32) {
    println!("{} K", celsius + 273);
}
