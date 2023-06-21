#[path="./request/get_following.rs"]
mod get_following;
#[path="./request/get_followers.rs"]
mod get_followers;

// Define the user and GitHub token constants
const USER: &str = "";
const GITHUB_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    // Create a new octorust client with the user and token
    let github: octorust::Client = octorust::Client::new(String::from("User"), octorust::auth::Credentials::Token(GITHUB_TOKEN.to_string())).unwrap();
    // Retrieve the list of following and followers
    let following: Vec<String> = get_following::get_following(github.clone(), USER).await;
    let followers: Vec<String> = get_followers::get_followers(github.clone(), USER).await;
    // Compare the following and followers lists
    for user in following {
        if !followers.contains(&user) {
            println!("- {}, don't follow you", user);
        }
    }
}
