/***************************************************************************************
 *   main.rs  --  This file is part of who_doesnt_follow_me.                           *
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

    show_users_dont_follow(&users_i_follow, &users_follow_me);
}
