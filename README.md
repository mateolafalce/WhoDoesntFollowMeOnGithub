<div align="center">

![logo](https://user-images.githubusercontent.com/98977436/269937105-0695c6a6-d1e8-4ce0-96a0-04ec26a56439.png)

# Who Doesn't follow me on github?


![gif](https://user-images.githubusercontent.com/98977436/269937066-bdfd8465-2992-48c4-85d6-13c4557adcf9.gif)

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
