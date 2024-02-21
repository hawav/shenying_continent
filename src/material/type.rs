/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

/// 材料信息
#[derive(Clone)]
pub struct Material {
    /// 材料名称
    pub name: &'static str,
    /// 材料说明
    pub desc: &'static str,
    /// 数量
    pub quantity: u32,
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} *{}", self.name, self.quantity)
    }
}
