pub fn get_github_token() -> String {
    println!("Enter your Github token:");
    let token: String = rpassword::read_password().unwrap();
    token
}
