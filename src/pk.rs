/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{
    creature::{Creature, Equipment},
    material, slow_println, take_time,
    weapon::{stone_sword, Quality},
    GameState,
};

// 战斗相关逻辑

/// 狩猎生物
pub fn hunt(state: &mut GameState, mut creature: Creature) {
    slow_println!("玩家接近 {} 触发战斗", creature.name);

    // 如果玩家装备了武器，使用武器的攻击，否则使用拳头。
    let ht = if let Some(weapon) = &state.equipment.weapon {
        weapon.ht
    } else {
        // 拳头 1 伤害
        1_00
    };

    let mut player = Creature {
        name: "玩家",
        hp: 20_00,
        ht,
        ht_factor: 100,
        equipment: Equipment {
            weapon: state.equipment.weapon.take(),
        },
    };

    match pk(&mut player, &mut creature) {
        PKResult::Win => {
            slow_println!("胜利！");

            take_time(1000);

            state.equipment.weapon = player.equipment.weapon.take();

            harvest_the_loot(state, &creature);

            match creature.name {
                "史莱姆" => state.history.remaining_slimes -= 1,
                "哥布林" => {
                    state.history.remaining_goblins -= 1;
                    if state.history.remaining_goblins == 0 {
                        // 解救矮人工匠师傅
                        take_time(2000);
                        slow_println!(
                            "发现一位被哥布林囚禁的矮人工匠师傅！我们将其解救，成为了朋友"
                        );
                        take_time(2000);

                        state.friends.push(Creature {
                            name: "矮人工匠师傅",
                            hp: 20_00,
                            ht: 0_00,
                            ht_factor: 100,
                            equipment: Equipment { weapon: None },
                        });
                    }
                }
                _ => {}
            }
        }
        PKResult::Draw => {
            slow_println!("平局");

            take_time(1000);

            state.equipment.weapon = player.equipment.weapon.take();
        }
        PKResult::Lose => {
            slow_println!("战败");

            take_time(1000);

            if let Some(weapon) = player.equipment.weapon.take() {
                slow_println!("您的武器 {} 被夺走了", weapon);
            }
        }
    };
}

/// PK 解说
fn pk_commentary(a: &Creature, b: &Creature, damage: u32) {
    match a.name {
        "哥布林" | "玩家" => {
            slow_println!("{}攻击{} 造成了 {} 伤害", a, b, damage as f32 / 100.0);
        }
        "矮人" => {
            slow_println!("{}对{}释放巫术 造成了 {} 伤害", a, b, damage as f32 / 100.0);
        }
        "史莱姆" => {
            slow_println!("{}粘住了{} 但并没有造成任何伤害", a, b);
        }
        _ => unreachable!(),
    }
}

fn pk(a: &mut Creature, b: &mut Creature) -> PKResult {
    loop {
        take_time(500);

        let a_damage = a.damage();
        let b_damage = b.damage();

        pk_commentary(a, b, a_damage);
        take_time(200);
        pk_commentary(b, a, b_damage);
        slow_println!();

        // if b_damage > 0 {
        //     slow_println!(
        //         "【{}】攻击【{}】造成了 {} 伤害",
        //         a.name,
        //         b.name,
        //         b_damage as f32 / 100.0
        //     );
        // }

        // if a_damage > 0 {
        //     slow_println!(
        //         "【{}】攻击【{}】 造成了 {} 伤害",
        //         b.name,
        //         a.name,
        //         a_damage as f32 / 100.0
        //     );
        // } else {
        //     slow_println!("【{}】粘住了【{}】但并没有造成任何伤害", b.name, a.name);
        // }

        b.hp = b.hp.saturating_sub(a_damage);
        a.hp = a.hp.saturating_sub(b_damage);

        let a_died = a.hp == 0;
        let b_died = b.hp == 0;

        if a_died && b_died {
            return PKResult::Draw;
        } else if a_died {
            return PKResult::Lose;
        } else if b_died {
            return PKResult::Win;
        }
    }
}

fn harvest_the_loot(state: &mut GameState, c: &Creature) {
    match c.name {
        "史莱姆" => {
            let loot = material::slime(3);
            slow_println!("获得 {}", loot);
            state.add_material_to_bag(loot);
        }
        "哥布林" => {
            let loot = stone_sword(Quality::Clumsy);
            slow_println!("获得 {}", loot);
            state.backpack.weapons.push(loot);
        }
        _ => unreachable!(),
    }
}

/// 对战结果
#[derive(Debug)]
pub enum PKResult {
    Win,
    Draw,
    Lose,
}
