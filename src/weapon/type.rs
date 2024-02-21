/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

/// 品质
#[derive(Clone)]
pub enum Quality {
    /// 粗劣的
    Clumsy,
    /// 普通的
    Ordinary,
    /// 精致的
    Exquisite,
    /// 非凡的
    Extraordinary,
}

impl Display for Quality {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Quality::Clumsy => write!(f, "粗劣的"),
            Quality::Ordinary => write!(f, "普通的"),
            Quality::Exquisite => write!(f, "精致的"),
            Quality::Extraordinary => write!(f, "非凡的"),
        }
    }
}

/// 武器
#[derive(Clone)]
pub struct Weapon {
    /// 武器名称
    pub name: &'static str,
    /// 武器介绍
    pub desc: &'static str,
    /// 武器伤害（单位 0.01）
    pub ht: u32,
    /// 武器等级（0～40）
    pub level: u32,
    /// 武器品质
    pub quality: Quality,
    /// 武器耐久度
    pub durability: u32,
}

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.quality, self.name)
    }
}

impl Weapon {
    pub(super) const fn new(
        name: &'static str,
        desc: &'static str,
        ht: u32,
        quality: Quality,
    ) -> Self {
        // 品质与寿命的关系
        // 拙劣 - 寿命 100 次
        // 普通 - 寿命 200 次
        // 精致 - 寿命 400 次
        // 非凡 - 寿命 800 次

        let durability = match quality {
            Quality::Clumsy => 100,
            Quality::Ordinary => 200,
            Quality::Exquisite => 400,
            Quality::Extraordinary => 800,
        };

        Self {
            name,
            desc,
            ht,
            level: 0, // 武器初始等级 0 级
            quality,
            durability,
        }
    }
}
