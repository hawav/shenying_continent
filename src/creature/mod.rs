/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod r#type;
pub use r#type::*;

use crate::weapon::{stone_sword, Quality};

/// 创建史莱姆生物
pub const fn slime() -> Creature {
    Creature {
        name: "史莱姆",
        hp: 5_00,
        ht: 0_00,
        ht_factor: 100,
        equipment: Equipment { weapon: None },
    }
}

/// 创建哥布林生物
pub const fn goblin() -> Creature {
    Creature {
        name: "哥布林",
        hp: 20_00,
        ht: 1_00,
        ht_factor: 80,
        equipment: Equipment {
            weapon: Some(stone_sword(Quality::Ordinary)),
        },
    }
}

/// 创建矮人生物
pub const fn dwarf() -> Creature {
    Creature {
        name: "矮人",
        hp: 20_00,
        ht: 10_00,
        ht_factor: 80,
        equipment: Equipment { weapon: None },
    }
}
