/*
Copyright (C) 2020-2021 Kunal Mehta <legoktm@member.fsf.org>

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
use eventstreams::{Event, StreamExt};

#[tokio::main]
async fn main() {
    let stream = eventstreams::stream();
    eventstreams::pin_mut!(stream);
    while let Some(event) = stream.next().await {
        match event {
            Event::Edit(edit) => {
                println!(
                    "{}: {} edited {}",
                    &edit.server_name, &edit.user, &edit.title
                );
            }
            Event::Log(log) => {
                println!(
                    "{}: {} performed {}/{} on {}",
                    &log.server_name,
                    &log.user,
                    &log.log_type,
                    &log.log_action,
                    &log.title
                );
            }
        }
    }
}
