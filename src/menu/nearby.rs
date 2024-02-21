/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{area::*, GameState};

use super::menu_dyn;

pub fn nearby_menu(state: &mut GameState) {
    menu_dyn(
        state,
        |_| "附近",
        |state| match AREAS[state.area].name {
            STR_HOMETOWN => hometown::nearby(),
            STR_SLIME_PLAIN => slime_plain::nearby(&state),
            STR_GOBLIN_JUNGLE => goblin_jungle::nearby(&state),
            STR_DWARF => dwarf_forest::nearby(),
            _ => unreachable!(),
        },
    );
}
