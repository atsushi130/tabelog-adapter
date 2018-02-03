
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use https_client::HttpsClientBuilder;
use std::io::Read;

pub struct TabelogSource;

impl<'a> TabelogSource {

    const BASE_URL: &'a str   = "https://tabelog.com/";
    const PARAMETERS: &'a str = "lid=hd_search1&vac_net=&svd=20180203&svt=1900&svps=2&hfc=1&Cat=MC&sw=";

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
        format!("{}tokyo/A1303/A130301/R4698/rstLst/?vs=1&sa={}&sk={}&{}",
                TabelogSource::BASE_URL,
                location,
                word,
                TabelogSource::PARAMETERS
        )
    }
}