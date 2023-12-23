/***************************************************************************************
 *   get_users_from_account.rs  --  This file is part of who_doesnt_follow_me.         *
 *                                                                                     *
 *   Copyright (C) 2023 Mateo Lafalce                                                  *
 *                                                                                     *
 *   who_doesnt_follow_me is free software: you can redistribute it and/or modify      *
 *   it under the terms of the GNU General Public License as published                 *
 *   by the Free Software Foundation, either version 3 of the License,                 *
 *   or (at your option) any later version.                                            *
 *                                                                                     *
 *   who_doesnt_follow_me is distributed in the hope that it will be useful,           *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty                       *
 *   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.                           *
 *   See the GNU General Public License for more details.                              *
 *                                                                                     *
 *   You should have received a copy of the GNU General Public License                 *
 *   along with this program.  If not, see http://www.gnu.org/licenses/.               *
 *                                                                                     *
 **************************************************************************************/

use crate::{
    error::{error, TOKEN_ERROR},
    user::User,
};
use octorust::{auth::Credentials, Client};

pub async fn get_users_from_account(type_user: &str, user: &User) -> Vec<String> {
    let user_str: String = "User".to_string();
    let token: Credentials = Credentials::Token(user.token.to_string());
    let client = Client::new(user_str, token).expect(TOKEN_ERROR);
    let users_request;

    match type_user {
        "follower" => users_request = client.users().list_all_followers_for_user(&user.name).await,
        _ => users_request = client.users().list_all_following_for_user(&user.name).await,
    };

    let mut vec_users: Vec<String> = [].to_vec();
    match users_request {
        Ok(response) => {
            for user in response {
                vec_users.push(user.login);
            }
            vec_users
        }
        Err(e) => {
            error(e.to_string());
            vec_users
        }
    }
}
