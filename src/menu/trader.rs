/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{menu::menu, slow_println, walking_simulate, Action, GameState};

pub fn trader_menu(state: &mut GameState) {
    walking_simulate();

    slow_println!("商人：物品高价回收。");

    menu(
        state,
        "与商人交互",
        vec![Action::SellSlime, Action::Backpack, Action::Back],
    );
}
