#[path = "./io/get_github_token.rs"]
mod get_github_token;
#[path = "./io/get_github_username.rs"]
mod get_github_username;
#[path = "./request/get_users_from_account.rs"]
mod get_users_from_account;
#[path = "./io/show_users_dont_follow.rs"]
mod show_users_dont_follow;

#[tokio::main]
async fn main() {
    let github_username: String = get_github_username::get_github_username();
    let github_token: String = get_github_token::get_github_token();
    let users_i_follow: Vec<String> = get_users_from_account::get_users_from_account(
        "following",
        &github_username,
        &github_token,
    )
    .await;
    let users_follow_me: Vec<String> = get_users_from_account::get_users_from_account(
        "follower", 
        &github_username, 
        &github_token
    )
    .await;
    show_users_dont_follow::show_users_dont_follow(users_i_follow, users_follow_me);
}
