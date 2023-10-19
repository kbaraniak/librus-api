use reqwest;

pub fn hello() -> String {
    println!("Welcome");
    "Hello\n".to_string()
}

pub async fn test_connect(){
    let _result = reqwest::get("https://example.com").await;
    match _result {
        Ok(_v) => println!("[Connect] Test passed"),
        Err(_e) => println!("[Connect] Network Error"),
    };
}


#[test]
fn test() {
    hello();
}