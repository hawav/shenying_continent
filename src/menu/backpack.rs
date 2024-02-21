/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt::Display;

use crate::{material::Material, select, slow_println, take_time, weapon::Weapon, GameState};

pub fn backpack_menu(state: &mut GameState) {
    #[derive(Clone)]
    enum Opt<'a> {
        Material(&'a Material),
        Weapon(&'a Weapon),
        Back,
    }

    impl<'a> Display for Opt<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Opt::Material(m) => m.fmt(f),
                Opt::Weapon(w) => w.fmt(f),
                Opt::Back => write!(f, "返回"),
            }
        }
    }

    let mut opts = state
        .backpack
        .materials
        .iter()
        .map(Opt::Material)
        .chain(state.backpack.weapons.iter().map(Opt::Weapon))
        .collect::<Vec<_>>();

    if opts.is_empty() {
        slow_println!("背包里没有任何东西");
    } else {
        opts.push(Opt::Back);
        while let Some(option) = select("背包", opts.clone()) {
            slow_println!(
                "{}",
                match option {
                    Opt::Material(m) => m.desc,
                    Opt::Weapon(w) => w.desc,
                    Opt::Back => break,
                }
            );
            take_time(1000);
        }
    }
}
