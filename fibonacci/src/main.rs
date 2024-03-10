mod fibonacci;

fn main() {
    let str_input_variable = "Enter a number Anna ".to_string();
    let no_of_elements : String = fibonacci::accept_input(str_input_variable);
    let no_of_elements_for_input: i32 =  match no_of_elements.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Problem while parsing ")
    };
    fibonacci::calculate_fibonacci(no_of_elements_for_input);
}