use std::io;
fn main() {
    println!("Convert the Degree Celcius to Fareheiht ");
    println!("Enter Temperature in Celcius  ");
    let mut celcius_temp  = String::new();
    io::stdin()
            .read_line(&mut celcius_temp)
            .expect("Failed to read line");

    println!("You Entered: {celcius_temp} in Celcius");
   let temp_celcius: f32 = match celcius_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Problem Parsing Float ")
    };
    println!("Temperature in Celcius = {temp_celcius}");
    let temp_farenheit :f32 = ( temp_celcius * 9.0/5.0)+32.0;
    println!("Temperature in Farenheit = {temp_farenheit}")
}
