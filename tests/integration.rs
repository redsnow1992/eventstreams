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

use eventstreams::EventStream;
use std::{thread, time::Duration};

fn wait(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

#[test]
fn test_connect() {
    let stream = EventStream::new();
    wait(5);
    stream.close();
}

#[test]
fn test_on_wiki() {
    let stream = EventStream::new();
    stream.on_wiki_edit("www.wikidata.org", |edit| {
        assert_eq!("www.wikidata.org", edit.server_name);
        assert_eq!("https://www.wikidata.org", edit.server_url);
    });
    stream.on_log(|log| {
        assert_eq!(format!("https://{}", log.server_name), log.server_url);
    });
    wait(10);
    stream.close();
}
