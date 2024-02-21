/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{menu::menu, slow_println, walking_simulate, Action, GameState};

pub fn artisan_menu(state: &mut GameState) {
    walking_simulate();

    slow_println!("工匠：刚造好的石剑，只要 10 金币！");

    menu(
        state,
        "与工匠交互",
        vec![
            Action::BuyStoneSword,
            Action::Equipment,
            Action::Backpack,
            Action::Back,
        ],
    );
}
