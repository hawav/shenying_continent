/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{area::AREAS, Action, GameState};

use super::menu_dyn;

pub fn main_menu(state: &mut GameState) {
    menu_dyn(
        state,
        |state| AREAS[state.area].name,
        |_| {
            vec![
                Action::Nearby,
                Action::Area,
                Action::Equipment,
                Action::Backpack,
            ]
        },
    );
}
