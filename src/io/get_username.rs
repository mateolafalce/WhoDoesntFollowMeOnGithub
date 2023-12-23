/***************************************************************************************
 *   get_username.rs  --  This file is part of who_doesnt_follow_me.                   *
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

pub fn get_username() -> String {
    std::process::Command::new("clear").status().unwrap();
    println!("Enter your Github username:");
    let mut github_username = String::new();
    std::io::stdin()
        .read_line(&mut github_username)
        .expect("Failed to read line");
    github_username.trim().to_string()
}
