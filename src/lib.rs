pub fn hello() -> String {
    println!("Welcome");
    "Hello\n".to_string()
}   


#[test]
fn test() {
    hello();
    println!("")
}