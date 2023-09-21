<div align="center">

![logo](logo.png)

# Who Doesn't follow me on github?

</div>

This script written in Rust is aimed at providing the growing professional with the numbers associated with their GitHub account. By comparing followers and following, you will immediately know who is not interested in sharing or following your work.

Copy and paste the following script and run the project!

```bash
git clone https://github.com/mateolafalce/WhoDoesntFollowMeOnGithub.git && cd WhoDoesntFollowMeOnGithub && cargo run --release
```

---

## Dependencies

```toml
[dependencies]
octorust = "0.3.2"
tokio = { version = "1", features = ["full"] }
rpassword = "7.2.0"
```

---

## main.rs

```rust
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
```

## License

This project is licensed under the `Apache License`, Version 2.0. You may not use this file except in compliance with the License. You may obtain a copy of the License at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
