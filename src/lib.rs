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
//! Clients can add listeners for edit and log entry events:
//! ```no_run
//! use eventstreams::EventStream;
//!
//! let stream = EventStream::new();
//! stream.on_edit(|edit| {
//!     println!(
//!         "{}: {} edited {}",
//!         &edit.server_name, &edit.user, &edit.title
//!     );
//! });
//! ```
//!
//! It's straightforward to filter events if you only care about a single wiki:
//! ```no_run
//! # use eventstreams::EventStream;
//! let stream = EventStream::new();
//! stream.on_wiki_edit("en.wikipedia.org", |edit| {
//!     println!(
//!         "{}: {} edited {}",
//!         &edit.server_name, &edit.user, &edit.title
//!     );
//! });
//! ```
mod types;

use serde_json::Value;
use sse_client::EventSource;
use std::marker::Send;
use types::{EditEvent, LogEvent};

pub struct EventStream {
    /// Allows manipulation/control of upstream [`sse_client:EventSource`](https://docs.rs/sse-client/1/sse_client/struct.EventSource.html).
    pub source: EventSource,
}

fn handle_line(line: &str) -> Option<Value> {
    if line.is_empty() {
        return None;
    }

    match serde_json::from_str(line) {
        Ok(val) => Some(val),
        // TODO: figure out why we sometimes get truncated lines
        Err(_) => None,
    }
}

impl EventStream {
    /// Create new `EventStream` instance
    pub fn new() -> EventStream {
        EventStream {
            source: EventSource::new(
                "https://stream.wikimedia.org/v2/stream/recentchange",
            )
            .unwrap(),
        }
    }

    /// Set a listener for edits on a specific wiki using the server name
    ///
    /// # Example
    /// ```no_run
    /// # use eventstreams::EventStream;
    /// # let stream = EventStream::new();
    /// stream.on_wiki_edit("www.wikidata.org", |edit| {
    ///     dbg!(edit);
    /// });
    /// ```
    pub fn on_wiki_edit<F>(&self, wiki: &'static str, listener: F)
    where
        F: Fn(EditEvent) + Send + 'static,
    {
        self.on_edit(move |edit| {
            if edit.server_name == wiki {
                listener(edit);
            }
        })
    }

    /// Set a listener for all edits
    ///
    /// # Example
    /// ```no_run
    /// # use eventstreams::EventStream;
    /// # let stream = EventStream::new();
    /// stream.on_edit(|edit| {
    ///     dbg!(edit);
    /// });
    /// ```
    pub fn on_edit<F>(&self, listener: F)
    where
        F: Fn(EditEvent) + Send + 'static,
    {
        self.source.on_message(move |message| {
            let data = handle_line(&message.data);
            if let Some(value) = data {
                if value["type"] == "edit" {
                    let edit: EditEvent =
                        serde_json::from_value(value).unwrap();
                    listener(edit);
                }
            }
        });
    }

    pub fn on_wiki_log<F>(&self, wiki: &'static str, listener: F)
    where
        F: Fn(LogEvent) + Send + 'static,
    {
        self.on_log(move |log| {
            if log.server_name == wiki {
                listener(log);
            }
        })
    }

    pub fn on_log<F>(&self, listener: F)
    where
        F: Fn(LogEvent) + Send + 'static,
    {
        self.source.on_message(move |message| {
            let data = handle_line(&message.data);
            if let Some(value) = data {
                if value["type"] == "log" {
                    let log: LogEvent = serde_json::from_value(value).unwrap();
                    listener(log);
                }
            }
        })
    }

    /// Close the connection. Wrapper around [`sse_client::EventSource#close`](https://docs.rs/sse-client/1/sse_client/struct.EventSource.html#method.close).
    pub fn close(&self) {
        self.source.close();
    }
}

impl Default for EventStream {
    fn default() -> Self {
        EventStream::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::handle_line;

    #[test]
    fn test_handle_line() {
        assert_eq!(None, handle_line(""));
        assert_eq!(None, handle_line("{invalid JSON"));
        assert_eq!(
            serde_json::json!({"foo": "bar"}),
            handle_line(r#"{"foo": "bar"}"#).unwrap()
        )
    }
}
