<div align="center">

![logo](logo.png)

# Who Doesn't follow me?

---

*The token to date is invalid, of course*

![gif](gif.gif)

</div>

This script written in Rust is aimed at providing the growing professional with the numbers associated with their GitHub account. By comparing followers and following, you will immediately know who is not interested in sharing or following your work.

If you're interested in trying out this script and you have a `Windows` operating system, feel free to download the repository and run the file `who_doesnt_follow_me_on_github.exe`. If you have an alternative operating system, you can download the repository and then compile it with `Cargo`.

## Dependencies

```toml
[dependencies]
octorust = "0.3.2"
tokio = { version = "1", features = ["full"] }
```

## Code

```rust
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

```

1. Inside the main function, the program starts by getting the username and token input from the user using the functions `username()` and `github_token()` respectively.

2. A `octorust::Client` object is created using the provided token input, allowing the program to make API requests to GitHub. Octorust is a Rust library for interacting with the GitHub API.

3. The program then proceeds to retrieve the list of users the provided username is following and the list of followers using the `get_following()` and `get_followers()` functions respectively. These functions likely make use of the `octorust::Client` object to fetch the data asynchronously.

4. After obtaining the lists of following and followers, the program compares the two using a loop. For each user in the `following` list, it checks if that user is not present in the `followers` list. If a user is found who does not follow the provided username, it prints a message indicating that they don't follow the user.

5. The program keeps track of the number of users who don't follow the provided username using the `users_amount` variable.

6. If no users are found who don't follow the provided username, a message is printed indicating that all users the username follows also follow them.

7. Finally, the program prompts the user to press Enter to exit and waits for input using `std::io::stdin().read_line()`.

## License

This project is licensed under the `Apache License`, Version 2.0. You may not use this file except in compliance with the License. You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
