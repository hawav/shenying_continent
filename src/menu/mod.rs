/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{select, take_time, Action, GameState};

/// 主菜单
pub mod main;

/// 地区菜单
pub mod area;

/// 背包菜单
pub mod backpack;

/// 装备菜单
pub mod equipment;

/// 武器选择菜单
pub mod weapon_select;

/// 附近菜单
pub mod nearby;

// ==== 家园

/// 工匠菜单
pub mod artisan;

/// 商人菜单
pub mod trader;

/// ==== Helper 函数 ====

fn menu(state: &mut GameState, title: &str, actions: Vec<Action>) {
    menu_dyn(state, |_| title, |_| actions.clone())
}

fn menu_dyn<'a, Title: Fn(&GameState) -> &'a str, Actions: Fn(&GameState) -> Vec<Action>>(
    state: &mut GameState,
    title: Title,
    actions: Actions,
) {
    loop {
        take_time(500);

        if let Some(action) = select(title(state), actions(state)) {
            match action {
                Action::Back => {
                    break;
                }
                _ => {
                    action.execute(state);
                }
            };
        } else {
            break;
        }
    }
}
