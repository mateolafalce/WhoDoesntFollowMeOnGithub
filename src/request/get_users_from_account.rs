pub async fn get_users_from_account(
    type_user: &str,
    github_username: &str,
    github_token: &str,
) -> Vec<String> {
    let mut vec_users: Vec<String> = [].to_vec();
    let users_request;
    let client: octorust::Client = octorust::Client::new(
        String::from("User"),
        octorust::auth::Credentials::Token(github_token.to_string()),
    )
    .unwrap();
    if type_user == "follower" {
        users_request = client
            .users()
            .list_all_followers_for_user(github_username)
            .await;
    } else {
        users_request = client
            .users()
            .list_all_following_for_user(github_username)
            .await;
    }
    match users_request {
        Ok(response) => {
            for user in response {
                vec_users.push(user.login);
            }
            return vec_users;
        }
        Err(_) => {
            return vec_users;
        }
    }
}
