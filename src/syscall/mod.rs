/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg_attr(not(no_std), path = "./std.rs")]
pub mod syscall_impl;

pub use syscall_impl::*;
