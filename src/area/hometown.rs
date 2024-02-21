/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    material, slow_println, text,
    weapon::{self, Quality},
    Action, GameState,
};

pub fn buy_stone_sword(state: &mut GameState) {
    if state.take_material_from_bag("金币", 10).is_ok() {
        let weapon = weapon::stone_sword(Quality::Ordinary);

        slow_println!("花费 {} 金币购买了 {}", 10, &weapon.name);

        state.backpack.weapons.push(weapon);
    } else {
        slow_println!("你的金币不够！先去收集些史莱姆粘液找商人换钱吧！");
    }
}

pub fn sell_slime(state: &mut GameState) {
    if let Some(num_of_slime) = state
        .backpack
        .materials
        .iter()
        .find(|m| m.name == "史莱姆粘液")
        .map(|m| m.quantity)
    {
        if num_of_slime == 0 {
            slow_println!("你没有史莱姆粘液");
        } else {
            loop {
                if let Ok(quantity) = match text(&format!(
                    "你有 {} 个史莱姆粘液，需要出售多少个？",
                    num_of_slime
                )) {
                    Some(q) => q,
                    None => break,
                }
                .parse()
                {
                    if state.take_material_from_bag("史莱姆粘液", quantity).is_ok() {
                        state.add_material_to_bag(material::coin(quantity));

                        slow_println!("出售 {} 个史莱姆粘液，获得 {} 个金币", quantity, quantity);

                        break;
                    } else {
                        slow_println!("你没有那么多史莱姆粘液");
                    }
                } else {
                    slow_println!("请输入阿拉伯数字");
                }
            }
        }
    } else {
        slow_println!("你没有史莱姆粘液");
    }
}

pub fn nearby() -> Vec<Action> {
    vec![Action::Artisan, Action::Trader, Action::Back]
}
