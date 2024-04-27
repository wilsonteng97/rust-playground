// [Advantage]:
// Using format! is usually the most succinct and readable way to combine strings.

// [Disadvantage]:
// It is usually not the most efficient way to combine strings 
// - a series of push operations on a mutable string is usually the most efficient
// (especially if the string has been pre-allocated to the expected size).

fn say_hello_no_format(name: &str) -> String {
    let mut greeting: String = "Hello ".to_owned();
    greeting.push_str(name);
    greeting.push('!');
    greeting
}

// does the same as "say_hello_no_format" with format macro
fn say_hello(name: &str) -> String {
    format!("Hello {name}!")
}


fn main() {
    println!("{}", say_hello_no_format("BobbyNoFormat"));
    println!("{}", say_hello("BobbyFormatted"));
}
