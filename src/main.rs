#[path = "./request/get_followers.rs"]
mod get_followers;
#[path = "./request/get_following.rs"]
mod get_following;
#[path = "./io/get_github_token.rs"]
mod get_github_token;
#[path = "./io/get_github_username.rs"]
mod get_github_username;
#[path = "./io/show_users_dont_follow.rs"]
mod show_users_dont_follow;

#[tokio::main]
async fn main() {
    let github_username: String = get_github_username::get_github_username();
    let github_token: String = get_github_token::get_github_token();
    let client: octorust::Client = octorust::Client::new(
        String::from("User"),
        octorust::auth::Credentials::Token(github_token),
    )
    .unwrap();
    let users_i_follow: Vec<String> =
        get_following::get_following(client.clone(), &github_username).await;
    let users_follow_me: Vec<String> =
        get_followers::get_followers(client.clone(), &github_username).await;
    show_users_dont_follow::show_users_dont_follow(users_i_follow, users_follow_me);
}
