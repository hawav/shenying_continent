/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod r#type;
pub use r#type::*;

/// 创建指定品质的石剑
pub const fn stone_sword(quality: Quality) -> Weapon {
    Weapon::new("石剑", "一把普通的石剑", 4_00, quality)
}
