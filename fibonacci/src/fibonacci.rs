use std::io;

pub fn calculate_fibonacci(no_of_elements:i32){
    println!("Calculating for No of Elements = {no_of_elements}");
    let  mut n1 :i32 = 0 ;
    let  mut n2 : i32 = 1 ;
    let mut counter = 1; 
    while counter <= no_of_elements{
        let sum :i32 = n1+n2;
        print!("{sum} ");
        // print!("{co/unter} ");
        counter += 1;
        n1 = n2;
        n2 = sum
    }

}

pub fn accept_input(prompt:String)-> String{
    println!("Inside Accept input method ");
    println!("{prompt}:");
    let mut input_str : String = String::new();
    io::stdin()
    .read_line(&mut input_str)
    .expect("Failed to read line");
    return input_str;
}