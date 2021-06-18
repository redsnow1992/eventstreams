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
use serde::Deserialize;
use serde_json::Value;

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

    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}{}/{}.php",
            self.server_url, self.server_script_path, path
        )
    }

    /// URL to the wiki's api.php ("[Action API](https://www.mediawiki.org/wiki/API:Main_page)") endpoint
    pub fn api_url(&self) -> String {
        self.endpoint("api")
    }

    fn title_for_url(&self) -> String {
        self.title.replace(" ", "_")
    }

    /// URL to the diff for this edit, formatted for human readability
    pub fn diff_url(&self) -> String {
        format!(
            "{}?title={}&diff={}",
            self.endpoint("index"),
            self.title_for_url(),
            self.revision.new
        )
    }

    /// URL to the diff for this edit, as short as possible
    pub fn short_diff_url(&self) -> String {
        format!("{}?diff={}", self.server_url, self.revision.new)
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
