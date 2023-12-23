/***************************************************************************************
 *   show_users_dont_follow.rs  --  This file is part of who_doesnt_follow_me.         *
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

pub fn show_users_dont_follow(users_i_follow: &Vec<String>, users_follow_me: &Vec<String>) {
    std::process::Command::new("clear").status().unwrap();
    let difference: Vec<_> = users_i_follow
        .iter()
        .filter(|&elem| !users_follow_me.contains(elem))
        .collect();
    let len: usize = difference.len();
    if len == 0 {
        println!("- All users that you follow, follow you too");
    } else {
        for user in difference {
            println!("- https://github.com/{} don't follow you", user);
        }
    }
}
