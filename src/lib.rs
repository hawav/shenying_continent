/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

/// 区域
pub mod area;
/// 史莱姆、哥布林、矮人等游戏生物
pub mod creature;
/// 史莱姆粘液、金币等游戏材料
pub mod material;
/// 菜单选项
pub mod menu;
/// 对战逻辑
pub mod pk;
/// 对外唯一系统调用接口
pub mod syscall;
/// 石剑等武器
pub mod weapon;

use core::fmt::Display;

use area::hometown::{buy_stone_sword, sell_slime};
use creature::{Creature, Equipment};
use material::Material;
use menu::{
    area::area_menu, artisan::artisan_menu, backpack::backpack_menu, equipment::equipment_menu,
    main::main_menu, nearby::nearby_menu, trader::trader_menu, weapon_select::weapon_select_menu,
};
use pk::hunt;
use syscall::*;
use weapon::Weapon;

/// 玩家背包
pub struct Bag {
    /// 材料
    pub materials: Vec<Material>,
    /// 武器
    pub weapons: Vec<Weapon>,
}

/// 菜单选项
#[derive(Clone)]
pub enum Action {
    // 打开背包
    Backpack,

    // 打开装备
    Equipment,
    // 卸下武器
    DropWeapon(&'static str),
    // 装备武器
    EquipWeapon,

    // 附近
    Nearby,
    // 区域
    Area,

    // 前往工匠
    Artisan,
    // 前往商人
    Trader,

    // ==== 工匠
    BuyStoneSword,

    // ==== 商人
    SellSlime,

    // 史莱姆
    Slime,
    // 哥布林
    Goblin,
    // 矮人
    Dwarf,

    // 返回
    Back,
}

impl Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // 通用菜单选项
            Action::Back => write!(f, "返回"),
            Action::Equipment => write!(f, "装备"),
            Action::Backpack => write!(f, "背包"),

            // 装备菜单选项
            Action::DropWeapon(weapon) => write!(f, "卸下武器（{}）", weapon),
            Action::EquipWeapon => write!(f, "装备武器"),

            // 主菜单选项
            Action::Nearby => write!(f, "附近"),
            Action::Area => write!(f, "区域"),

            // 家园附近选项
            Action::Artisan => write!(f, "工匠"),
            Action::Trader => write!(f, "商人"),

            // 工匠菜单选项
            Action::BuyStoneSword => write!(f, "购买石剑"),

            // 商人菜单选项
            Action::SellSlime => write!(f, "出售史莱姆粘液"),

            // 附近生物选项
            Action::Slime => write!(f, "史莱姆"),
            Action::Goblin => write!(f, "哥布林"),
            Action::Dwarf => write!(f, "矮人"),
        }
    }
}

impl Action {
    fn execute(self, state: &mut GameState) {
        match self {
            // 进入对应菜单
            Action::Equipment => equipment_menu(state),
            Action::Backpack => backpack_menu(state),
            Action::Nearby => nearby_menu(state),
            Action::Area => area_menu(state),

            // 装备菜单
            Action::DropWeapon(_) => state.equipment.weapon = None,
            Action::EquipWeapon => state.equipment.weapon = weapon_select_menu(state),

            // 商人菜单
            Action::Trader => trader_menu(state),
            Action::SellSlime => sell_slime(state),

            // 工匠菜单
            Action::Artisan => artisan_menu(state),
            Action::BuyStoneSword => buy_stone_sword(state),

            // 狩猎生物
            Action::Slime => hunt(state, creature::slime()),
            Action::Goblin => hunt(state, creature::goblin()),
            Action::Dwarf => {
                // 如果发现矮人工匠师傅是我们的朋友，矮人伙伴就会对我们友善，否则就会攻击我们。
                if !state.history.win
                    && state
                        .friends
                        .iter()
                        .find(|f| f.name == "矮人工匠师傅")
                        .is_some()
                {
                    state.history.win = true;

                    slow_println!("矮人：An'gosh-dun khaz'modan dor-bardum garmok!（感谢你救回我们的矮人工匠师傅！）");

                    take_time(3000);

                    panic!("累了，写不动了，作者需要鼓励才有动力继续开发...");
                } else {
                    hunt(state, creature::dwarf());
                }
            }

            // Back 应该提前匹配并 break 出循环，而不是通过 execute() 执行
            Action::Back => unreachable!(),
        }
    }
}

/// 记录世界中生物的数量、已探索的区域等历史状态
pub struct History {
    /// 已解锁的区域下标
    pub known_area: usize,
    /// 区域中剩余史莱姆数量
    pub remaining_slimes: usize,
    /// 区域中剩余哥布林数量
    pub remaining_goblins: usize,
    /// 游戏是否胜利
    pub win: bool,
}

/// 记录玩家的背包、装备等游戏状态
pub struct GameState {
    /// 当前区域下标
    pub area: usize,
    /// 背包
    pub backpack: Bag,
    /// 装备
    pub equipment: Equipment,
    /// 认识的朋友
    pub friends: Vec<Creature>,
    /// 游戏历史
    pub history: History,
}

/// 标识背包中没有足够数量材料
pub struct NotEnoughInTheBag;

impl GameState {
    // 将材料放入背包
    pub fn add_material_to_bag(&mut self, material: Material) {
        if let Some(existing_item) = self
            .backpack
            .materials
            .iter_mut()
            .find(|i| i.name == material.name)
        {
            existing_item.quantity += material.quantity;
        } else {
            self.backpack.materials.push(material);
        }
    }

    // 从背包中取出指定数量的材料，成功取出返回 Ok(()) 数量不足无法取出则返回 Err(())
    pub fn take_material_from_bag(
        &mut self,
        name: &str,
        quantity: u32,
    ) -> Result<(), NotEnoughInTheBag> {
        if let Some((i, material)) = self
            .backpack
            .materials
            .iter_mut()
            .enumerate()
            .find(|(_, m)| m.name == name)
        {
            match material.quantity.cmp(&quantity) {
                core::cmp::Ordering::Less => Err(NotEnoughInTheBag),
                core::cmp::Ordering::Equal => {
                    self.backpack.materials.remove(i);
                    Ok(())
                }
                core::cmp::Ordering::Greater => {
                    material.quantity -= quantity;
                    Ok(())
                }
            }
        } else {
            Err(NotEnoughInTheBag)
        }
    }
}

fn take_time(ms: u64) {
    sleep_ms(ms);
}

/// 与 println 同样的语法，一个字一个字地打印字符串
#[macro_export]
macro_rules! slow_println {
    () => {
        slow_println("")
    };
    ($($arg:tt)*) => {
        {
            slow_println(&format!($($arg)*))
        }
    };
}

/// 模拟路上花费的时间
fn walking_simulate() {
    slow_println!("正在前往...");
    take_time(500);
}

pub struct Game;

impl Game {
    pub fn start() {
        // 开场白
        slow_println!(
            "你降临到了奇幻的神影大陆，为了带领人族在这个奇幻的世界走向巅峰，勇敢地去探索未知吧！"
        );

        let mut state = GameState {
            area: 0,
            backpack: Bag {
                materials: Vec::new(),
                weapons: Vec::new(),
            },
            equipment: Equipment { weapon: None },
            history: History {
                known_area: 1,
                remaining_slimes: 7,
                remaining_goblins: 4,
                win: false,
            },
            friends: Vec::new(),
        };

        loop {
            main_menu(&mut state);
            if select("确定要退出吗？", vec!["是"]).is_some() {
                break;
            }
        }
    }
}
