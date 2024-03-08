use std::io;

pub fn calculate_fibonacci(no_of_elements:i32){
    println!("Calculating for No of Elements = {no_of_elements}")
}

pub fn accept_input(prompt:String)-> String{
    println!("Inside Accept input method ");
    println!("{}",prompt);
    let mut input_str : String = String::new();
    io::stdin()
    .read_line(&mut input_str)
    .expect("Failed to read line");
    return input_str;
}