
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use https_client::HttpsClientBuilder;
use std::io::Read;
use super::DateTimeToString;
use chrono::Local;

pub struct TabelogSource;

impl<'a> TabelogSource {

    const BASE_URL: &'a str   = "https://tabelog.com/";
    const PARAMETERS: &'a str = "LstKind=1&voluntary_search=1&lid=top_navi1&sa=&vac_net=&&svps=2&svt=2030&hfc=1&form_submit=&area_datatype=RailroadStation&area_id=4698&key_datatype=&key_id=";

    pub fn get_source(&self, location: &str, word: &str) -> String {

        let url = self.build_url(location, word);

        let mut response = String::new();
        HttpsClientBuilder::build()
            .get(&url)
            .send()
            .unwrap()
            .read_to_string(&mut response)
            .unwrap();

        response
    }

    fn build_url(&self, location: &str, word: &str) -> String {
        format!("{}rst/rstsearch/?sa_input={}&sk={}&svd={}&{}",
                TabelogSource::BASE_URL,
                location,
                word,
                Local::now().to_format_string(),
                TabelogSource::PARAMETERS
        )
    }
}