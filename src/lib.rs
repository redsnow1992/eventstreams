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
//! # eventstreams
//!
//! The `eventstreams` crate provides a convenient, typed, wrapper around
//! Wikimedia's  [EventStreams](https://wikitech.wikimedia.org/wiki/Event_Platform/EventStreams)
//! live recent changes feed.
//!
//! ```no_run
//! # async fn doc() {
//! use eventstreams::{Event,StreamExt};
//!
//! let stream = eventstreams::stream();
//! eventstreams::pin_mut!(stream);
//! while let Some(event) = stream.next().await {
//!    match event {
//!        Event::Edit(edit) => {
//!            println!(
//!                "{}: {} edited {}",
//!                &edit.server_name, &edit.user, &edit.title
//!            );
//!        }
//!        Event::Log(log) => {
//!            println!(
//!                "{}: {} performed {}/{} on {}",
//!                &log.server_name,
//!                &log.user,
//!                &log.log_type,
//!                &log.log_action,
//!                &log.title
//!            );
//!        }
//!    }
//! }
//! # }
//! ```
mod types;

use async_stream::stream;
pub use futures::{Stream, StreamExt};
pub use futures_util::pin_mut;
use serde_json::Value;
use surf_sse::{Event as SSEEvent, EventSource};
pub use types::{EditEvent, Event, LogEvent};

fn handle_event(event: SSEEvent) -> Option<Event> {
    if event.data.is_empty() {
        return None;
    }
    let value: Value = match serde_json::from_str(&event.data) {
        Ok(value) => value,
        Err(_) => return None,
    };
    if value["type"] == "log" {
        Some(Event::Log(serde_json::from_value(value).unwrap()))
    } else if value["type"] == "edit" {
        Some(Event::Edit(serde_json::from_value(value).unwrap()))
    } else {
        None
    }
}

pub fn stream() -> impl Stream<Item = Event> {
    let source = EventSource::new(
        "https://stream.wikimedia.org/v2/stream/recentchange"
            .parse()
            .unwrap(),
    );
    stream! {
        for await event in source {
            if let Some(event) = handle_event(event.unwrap()) {
                yield event;
            }
        }
    }
}
