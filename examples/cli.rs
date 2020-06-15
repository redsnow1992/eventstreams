/*
Copyright (C) 2020 Kunal Mehta <legoktm@member.fsf.org>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! Dump the EventStream feed to your terminal.
//!
//! Run with `cargo run --example cli`
use eventstreams::EventStream;
use std::thread;

fn main() {
    let stream = EventStream::new();
    stream.on_edit(|edit| {
        println!(
            "{}: {} edited {}",
            &edit.server_name, &edit.user, &edit.title
        );
    });
    stream.on_log(|log| {
        println!(
            "{}: {} did {}/{} on {}",
            &log.server_name,
            &log.user,
            &log.log_type,
            &log.log_action,
            &log.title
        )
    });
    stream.source.on_open(|| {
        println!("Connected.");
    });
    thread::park();
}
