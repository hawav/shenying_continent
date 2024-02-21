/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{Action, GameState};

pub fn nearby(state: &GameState) -> Vec<Action> {
    vec![Action::Goblin; state.history.remaining_goblins]
}
