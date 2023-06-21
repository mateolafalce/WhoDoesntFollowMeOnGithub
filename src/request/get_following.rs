pub async fn get_following(
    github: octorust::Client,
    user: &str,
) -> Vec<String> {
    // Fetch the list of users that the given user is following
    let following_response = github.users().list_all_following_for_user(user).await;
    let mut following: Vec<String> = [].to_vec();
    match following_response {
        Ok(resp) => {
            // Iterate through the response and add each user's login to the 'following' vector
            for user in resp {
                following.push(user.login);
            }
            return following; // Return the populated 'following' vector
        }
        Err(err) => {
            // Print the request error if there was an issue
            println!("Request error: {}", err);
            return following; // Return the empty 'following' vector
        }
    }
}
