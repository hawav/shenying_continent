/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt::Display;

use crate::{select, slow_println, weapon::Weapon, GameState};

pub fn weapon_select_menu(state: &mut GameState) -> Option<Weapon> {
    struct Opt {
        weapon: Weapon,
        idx: usize,
    }

    impl Display for Opt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.weapon.fmt(f)
        }
    }

    let opts: Vec<_> = state
        .backpack
        .weapons
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, weapon)| Opt { weapon, idx })
        .collect();

    if opts.is_empty() {
        slow_println!("背包里没有武器");
        None
    } else {
        select("选择武器", opts).map(|opt| state.backpack.weapons.remove(opt.idx))
    }
}
