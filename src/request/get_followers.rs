pub async fn get_followers(github: octorust::Client, user: &str) -> Vec<String> {
    let followers_response = github.users().list_all_followers_for_user(user).await;
    let mut followers: Vec<String> = [].to_vec();
    match followers_response {
        Ok(resp) => {
            for user in resp {
                followers.push(user.login); // Add follower's login to the `followers` vector
            }
            return followers; // Return the list of followers
        }
        Err(err) => {
            println!("Request error: {}", err); // Print the error message if there was an error
            return followers; // Return an empty list of followers
        }
    }
}
