#[path="./request/get_following.rs"]
mod get_following;
#[path="./request/get_followers.rs"]
mod get_followers;
#[path="./io/username.rs"]
mod username;
#[path="./io/github_token.rs"]
mod github_token;

#[tokio::main]
async fn main() {
    //Get username
    let mut user_input: String = username::username();
    //Get token
    let token_input: String = github_token::github_token();
    let client: octorust::Client = octorust::Client::new(String::from("User"), octorust::auth::Credentials::Token(token_input.to_string())).unwrap();
    // Retrieve the list of following and followers
    let following: Vec<String> = get_following::get_following(client.clone(), &user_input).await;
    let followers: Vec<String> = get_followers::get_followers(client.clone(), &user_input).await;
    // Compare the following and followers lists
    let mut users_amount = 0;
    println!("------------------------------------------");
    for user in following {
        if !followers.contains(&user) {
            println!("- {}, don't follow you", user);
            users_amount += 1;
        }
    }
    if users_amount == 0 {
        println!("- All users that you follow, follow you too");
    }
    println!("------------------------------------------");
    println!("Press Enter to exit...");
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
}
