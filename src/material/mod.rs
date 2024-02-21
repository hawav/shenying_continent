/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod r#type;
pub use r#type::*;

/// 创建指定数量的金币材料
pub fn coin(quantity: u32) -> Material {
    Material {
        name: "金币",
        desc: "金闪闪的通用货币",
        quantity,
    }
}

/// 创建指定数量的史莱姆粘液材料
pub fn slime(quantity: u32) -> Material {
    Material {
        name: "史莱姆粘液",
        desc: "非常适合用来粘东西，可以找商人回收换金币",
        quantity,
    }
}
