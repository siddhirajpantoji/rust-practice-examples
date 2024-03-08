fn main() {
    let mut x:i32 = 5;
    another_function(x);
    x = increment_number(x);
    another_function(x);

}
/**
 * This is a sample Multi line comment 
 * 
 * Can do anything over this 
 */
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn increment_number(x:i32)-> i32{
x+1
}