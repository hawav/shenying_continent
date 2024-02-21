/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt::Display;

use crate::weapon::Weapon;

/// 生物信息
pub struct Creature {
    /// 生物名称
    pub name: &'static str,
    /// 血量（单位 0.01）
    pub hp: u32,
    /// 伤害（单位 0.01）
    pub ht: u32,
    /// 伤害倍率（单位 1%）
    pub ht_factor: u32,
    /// 装备
    pub equipment: Equipment,
}

/// 生物身上的装备
pub struct Equipment {
    /// 手持武器
    pub weapon: Option<Weapon>,
}

impl Creature {
    /// 计算生物攻击伤害
    pub fn damage(&self) -> u32 {
        self.ht
            .max(self.equipment.weapon.as_ref().map(|w| w.ht).unwrap_or(0))
            * self.ht_factor
            / 100
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "【{}({})】", self.name, self.hp as f32 / 100.0)
    }
}
