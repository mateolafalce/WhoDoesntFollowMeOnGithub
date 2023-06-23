pub fn username() -> String {
    println!("------------------------------------------");
    println!("Enter your Github username:");
    println!("------------------------------------------");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let username = user_input.trim().to_string();
    username
}
