pub fn github_token() -> String {
    println!("------------------------------------------");
    println!("Enter your Github token");
    println!("------------------------------------------");
    let mut token_input = String::new();
    std::io::stdin().read_line(&mut token_input).expect("Failed to read line");
    let token: String = token_input.trim().to_string();
    token
}
