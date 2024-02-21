/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{Action, GameState};

use super::menu_dyn;

pub fn equipment_menu(state: &mut GameState) {
    menu_dyn(
        state,
        |_| "装备",
        |state| {
            if let Some(weapon) = state.equipment.weapon.as_ref() {
                vec![Action::DropWeapon(weapon.name), Action::Back]
            } else {
                vec![Action::EquipWeapon, Action::Back]
            }
        },
    )
}
