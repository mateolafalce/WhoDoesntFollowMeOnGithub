<div align="center">

![logo](https://user-images.githubusercontent.com/98977436/269937105-0695c6a6-d1e8-4ce0-96a0-04ec26a56439.png)

![License](https://img.shields.io/github/license/mateolafalce/WhoDoesntFollowMeOnGithub)
![GitHub repo size](https://img.shields.io/github/repo-size/mateolafalce/WhoDoesntFollowMeOnGithub)

# Who Doesn't follow me on github?

![gif](https://user-images.githubusercontent.com/98977436/269937066-bdfd8465-2992-48c4-85d6-13c4557adcf9.gif)

</div>

This script written in Rust is aimed at providing the growing professional with the numbers associated with their GitHub account. By comparing followers and following, you will immediately know who is not interested in sharing or following your work.

## Install

Copy and paste the following script and run the project!

```bash
cargo install --git https://github.com/mateolafalce/WhoDoesntFollowMeOnGithub.git
```

## Usage

Just `wdfm` in your terminal!

---

## Dependencies

```toml
[dependencies]
octorust = "0.3.2"
tokio = { version = "1.35.1", features = ["full"] }
rpassword = "7.3.1"
```

---

## main.rs

```rust
mod error;
mod io;
mod request;
mod user;

use crate::{
    io::{
        get_token::get_token, get_username::get_username,
        show_users_dont_follow::show_users_dont_follow,
    },
    request::get_users_from_account::get_users_from_account,
    user::User,
};

#[tokio::main]
async fn main() {
    let user: User = User {
        name: get_username(),
        token: get_token(),
    };

    let users_i_follow: Vec<String> = get_users_from_account("following", &user).await;
    let users_follow_me: Vec<String> = get_users_from_account("follower", &user).await;

    show_users_dont_follow(users_i_follow, users_follow_me);
}
```
