/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt::Display;

use crate::{area::AREAS, select, slow_println, take_time, GameState};

pub fn area_menu(state: &mut GameState) {
    slow_println!("打开地图...");
    take_time(1000);

    struct AreaOption {
        name: &'static str,
        idx: Option<usize>,
    }

    impl Display for AreaOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.name.fmt(f)
        }
    }

    let mut areas: Vec<_> = AREAS
        .iter()
        .take(state.history.known_area)
        .enumerate()
        .map(|(idx, a)| AreaOption {
            name: a.name,
            idx: Some(idx),
        })
        .collect();

    if state.history.known_area < AREAS.len() {
        areas.push(AreaOption {
            name: "未知区域",
            idx: None,
        });
    }

    if let Some(area) = select("地图", areas) {
        if let Some(area_idx) = area.idx {
            // 前往已知地区

            if area_idx == state.area {
                slow_println!("已经在 {} 了", area);
                return;
            }

            state.area = area_idx;
        } else {
            // 前往未知区域
            state.area = state.history.known_area;
            state.history.known_area += 1;
        }

        slow_println!("正在前往 {}...", area);
        take_time(1000);
    }
}
