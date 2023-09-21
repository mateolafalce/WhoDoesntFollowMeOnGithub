pub fn show_users_dont_follow(users_i_follow: Vec<String>, users_follow_me: Vec<String>) {
    std::process::Command::new("clear").status().unwrap();
    let mut users_amount: u16 = 0;
    for user in users_i_follow {
        if !users_follow_me.contains(&user) {
            println!("- https://github.com/{} don't follow you", user);
            users_amount += 1;
        }
    }
    if users_amount == 0 {
        println!("- All users that you follow, follow you too");
    }
}
