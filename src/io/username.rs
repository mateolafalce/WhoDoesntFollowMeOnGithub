pub fn username() -> String {
    // Prompt the user to enter their Github username
    println!("------------------------------------------");
    println!("Enter your Github username:");
    println!("------------------------------------------");
    // Read the user's input from the standard input
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    // Remove any trailing newline characters and convert the input to a String
    let username = user_input.trim().to_string();
    username
}
