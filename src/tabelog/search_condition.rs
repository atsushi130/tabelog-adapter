
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

pub struct SearchCondition {
    pub location: String,
    pub word: String
}

impl SearchCondition {
    pub fn from(location: &str, word: &str) -> Self {
        SearchCondition {
            location: location.to_string(),
            word: word.to_string()
        }
    }
}