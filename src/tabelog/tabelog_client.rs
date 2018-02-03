
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use tabelog::tabelog_source::TabelogSource;
use regex::Regex;

pub struct TabelogClient;

pub type SearchResult = Vec<String>;
impl TabelogClient {

    pub fn search(&self, location: &str, word: &str) -> SearchResult {
        let source = TabelogSource.get_source(location, word);
        let regex = Regex::new("data-detail-url=\"(?P<url>.*)\" data-rst-id").unwrap();

        regex.captures_iter(&source).map(|capture| {
            capture["url"].to_string()
        }).collect()
    }
}