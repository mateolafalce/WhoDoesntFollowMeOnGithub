pub fn github_token() -> String {
    // Prompt the user to enter their Github token
    println!("------------------------------------------");
    println!("Enter your Github token");
    println!("------------------------------------------");
    // Create a mutable string to store the user input
    let mut token_input = String::new();
    // Read the user input from the standard input
    std::io::stdin().read_line(&mut token_input).expect("Failed to read line");
    // Trim any leading or trailing whitespace and convert the input to a String
    let token: String = token_input.trim().to_string();
    token
}
