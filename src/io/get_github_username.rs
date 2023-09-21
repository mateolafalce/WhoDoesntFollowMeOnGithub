pub fn get_github_username() -> String {
    std::process::Command::new("clear").status().unwrap();
    println!("Enter your Github username:");
    let mut github_username = String::new();
    std::io::stdin()
        .read_line(&mut github_username)
        .expect("Failed to read line");
    github_username.trim().to_string()
}
