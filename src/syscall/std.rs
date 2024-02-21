/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt::Display;
use std::{io::Write, thread::sleep, time::Duration};

use inquire::{Select, Text};

/// 毫秒延时
pub fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms))
}

/// 一个一个字符打印字符串
pub fn slow_println(str: &str) {
    for ch in str.chars() {
        print!("{}", ch);
        std::io::stdout().flush().unwrap();
        sleep_ms(100);
    }
    println!();
}

/// 让用户做出选择
pub fn select<T: Display>(title: &str, options: Vec<T>) -> Option<T> {
    Select::new(title, options).prompt().ok()
}

/// 让用户输入文本
pub fn text(title: &str) -> Option<String> {
    Text::new(title).prompt().ok()
}
