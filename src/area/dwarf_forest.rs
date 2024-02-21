/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Action;

pub fn nearby() -> Vec<Action> {
    vec![Action::Dwarf, Action::Dwarf, Action::Dwarf, Action::Back]
}
