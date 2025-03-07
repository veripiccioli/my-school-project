use std::fs;
fn main() {
    let path = "my-school-project";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    println!("{}", contents);
}
