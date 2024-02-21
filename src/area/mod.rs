/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// 4.矮人森林
pub mod dwarf_forest;
/// 3.哥布林丛林
pub mod goblin_jungle;
/// 1.家园
pub mod hometown;
/// 2.史莱姆平原
pub mod slime_plain;

#[doc(hidden)]
pub const STR_HOMETOWN: &str = "家园";
#[doc(hidden)]
pub const STR_SLIME_PLAIN: &str = "史莱姆平原";
#[doc(hidden)]
pub const STR_GOBLIN_JUNGLE: &str = "哥布林丛林";
#[doc(hidden)]
pub const STR_DWARF: &str = "矮人森林";

#[doc(hidden)]
pub const AREA_HOMETOWN: Area = Area { name: STR_HOMETOWN };
#[doc(hidden)]
pub const AREA_SLIME_PLAIN: Area = Area {
    name: STR_SLIME_PLAIN,
};
#[doc(hidden)]
pub const AREA_GOBLIN_JUNGLE: Area = Area {
    name: STR_GOBLIN_JUNGLE,
};
#[doc(hidden)]
pub const AREA_DWARF: Area = Area { name: STR_DWARF };

/// 这里是所有区域，为了保留玩家探索的乐趣，玩家只能访问前 known_area 个区域，新区域需要探索才能解锁，known_area 在 GameState 结构体中定义。
pub const AREAS: [Area; 4] = [
    AREA_HOMETOWN,
    AREA_SLIME_PLAIN,
    AREA_GOBLIN_JUNGLE,
    AREA_DWARF,
];

/// 区域信息
#[derive(PartialEq)]
pub struct Area {
    pub name: &'static str,
}
