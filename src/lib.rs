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
//!
//! # Optional features
//! An optional `mediawiki-api` feature provides tighter integration with the
//! [`mediawiki`](https://docs.rs/mediawiki/) crate.
#[cfg(feature = "mediawiki-api")]
use mediawiki::{api::Api, title::Title};
use serde::Deserialize;
use serde_json::Value;
use sse_client::EventSource;
use std::marker::Send;

/// Represents an edit
#[derive(Clone, Debug, Deserialize)]
pub struct EditEvent {
    #[serde(rename = "$schema")]
    schema: String,
    // TODO: figure out a better structure for this
    meta: EventMeta,
    /// Revision ID ([rev_id](https://www.mediawiki.org/wiki/Manual:Revision_table#rev_id))
    pub id: u32,
    #[serde(rename = "type")]
    type_: String,
    /// Namespace ID
    pub namespace: i32,
    /// Prefixed title (includes namespace name)
    pub title: String,
    /// Edit summary ([comment_text](https://www.mediawiki.org/wiki/Manual:Comment_table#comment_text))
    pub comment: String,
    /// HTML-parsed version of [`comment`](EditEvent#structfield.comment)
    pub parsedcomment: String,
    /// Unix timestamp
    pub timestamp: u32,
    /// Username ([actor_name](https://www.mediawiki.org/wiki/Manual:Actor_table#actor_name))
    pub user: String,
    /// Whether the edit was flagged as by a bot ([rc_bot](https://www.mediawiki.org/wiki/Manual:Recentchanges_table#rc_bot))
    pub bot: bool,
    minor: Option<bool>,
    patrolled: Option<bool>,
    /// Length in bytes of new revision, and potentially old revision
    pub length: EventLength,
    /// Revision ID of new revision, and potentially old revision
    pub revision: EventRevision,
    /// URL of wiki with protocol, e.g. `https://www.wikidata.org`
    pub server_url: String,
    /// Domain of wiki with no protocol, e.g. `www.wikidata.org` or `en.wikipedia.org`
    pub server_name: String,
    /// Base URL path of wiki ([$wgScriptPath](https://www.mediawiki.org/wiki/Manual:$wgScriptPath))
    pub server_script_path: String,
    /// Internal database name (usually [$wgDBname](https://www.mediawiki.org/wiki/Manual:$wgDBname))
    pub wiki: String,
}

impl EditEvent {
    /// Whether the edit is marked as minor
    pub fn is_minor(&self) -> bool {
        self.minor.unwrap_or(false)
    }

    /// Whether the edit has been marked as patrolled
    pub fn is_patrolled(&self) -> bool {
        self.patrolled.unwrap_or(false)
    }

    /// URL to the wiki's api.php ("[Action API](https://www.mediawiki.org/wiki/API:Main_page)") endpoint
    pub fn api_url(&self) -> String {
        format!("{}{}/api.php", self.server_url, self.server_script_path)
    }

    /// Get a [`mediawiki::api::Api`](https://docs.rs/mediawiki/latest/mediawiki/api/struct.Api.html) instance
    /// # Optional
    /// Requires `mediawiki-api` feature
    #[cfg(feature = "mediawiki-api")]
    pub fn api(&self) -> Api {
        Api::new(&self.api_url()).unwrap()
    }

    /// Get a [`mediawiki::title::Title`](https://docs.rs/mediawiki/latest/mediawiki/title/struct.Title.html) instance
    /// # Optional
    /// Requires `mediawiki-api` feature
    #[cfg(feature = "mediawiki-api")]
    pub fn get_title(&self) -> Title {
        Title::new_from_full(&self.title, &self.api())
    }
}

/// Represents a log entry
#[derive(Clone, Debug, Deserialize)]
pub struct LogEvent {
    #[serde(rename = "$schema")]
    schema: String,
    meta: EventMeta,
    #[serde(rename = "type")]
    type_: String,
    /// Namespace ID
    pub namespace: i32,
    /// Prefixed title (includes namespace name)
    pub title: String,
    /// Edit summary ([comment_text](https://www.mediawiki.org/wiki/Manual:Comment_table#comment_text))
    pub comment: String,
    /// HTML-parsed version of [`comment`](EditEvent#structfield.comment)
    pub parsedcomment: String,
    /// Unix timestamp
    pub timestamp: u32,
    /// Username ([actor_name](https://www.mediawiki.org/wiki/Manual:Actor_table#actor_name))
    pub user: String,
    /// Whether the edit was flagged as by a bot ([rc_bot](https://www.mediawiki.org/wiki/Manual:Recentchanges_table#rc_bot))
    pub bot: bool,
    pub log_id: u32,
    pub log_type: String,
    pub log_action: String,
    pub log_params: Value,
    pub log_action_comment: String,
    /// URL of wiki with protocol, e.g. `https://www.wikidata.org`
    pub server_url: String,
    /// Domain of wiki with no protocol, e.g. `www.wikidata.org` or `en.wikipedia.org`
    pub server_name: String,
    /// Base URL path of wiki ([$wgScriptPath](https://www.mediawiki.org/wiki/Manual:$wgScriptPath))
    pub server_script_path: String,
    /// Internal database name (usually [$wgDBname](https://www.mediawiki.org/wiki/Manual:$wgDBname))
    pub wiki: String,
}

impl LogEvent {
    /// URL to the wiki's api.php ("[Action API](https://www.mediawiki.org/wiki/API:Main_page)") endpoint
    pub fn api_url(&self) -> String {
        format!("{}{}/api.php", self.server_url, self.server_script_path)
    }

    /// Get a [`mediawiki::api::Api`](https://docs.rs/mediawiki/latest/mediawiki/api/struct.Api.html) instance
    /// # Optional
    /// Requires `mediawiki-api` feature
    #[cfg(feature = "mediawiki-api")]
    pub fn api(&self) -> Api {
        Api::new(&self.api_url()).unwrap()
    }

    /// Get a [`mediawiki::title::Title`](https://docs.rs/mediawiki/latest/mediawiki/title/struct.Title.html) instance
    /// # Optional
    /// Requires `mediawiki-api` feature
    #[cfg(feature = "mediawiki-api")]
    pub fn get_title(&self) -> Title {
        Title::new_from_full(&self.title, &self.api())
    }
}

/// Length in bytes of new revision, and potentially old revision
#[derive(Clone, Debug, Deserialize)]
pub struct EventLength {
    /// Length of old revision, in bytes
    pub old: Option<u32>,
    /// Length of new revision, in bytes
    pub new: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EventRevision {
    /// Revision ID for old revision
    pub old: Option<u32>,
    /// Revision ID for new revision
    pub new: u32,
}

#[derive(Clone, Debug, Deserialize)]
struct EventMeta {
    uri: String,
    request_id: String,
    id: String,
    dt: String,
    domain: String,
    stream: String,
    topic: String,
    partition: u32,
    offset: u32,
}

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
