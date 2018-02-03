
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate regex;
extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;

mod https_client;
use https_client::HttpsClientBuilder;

use regex::Regex;
use std::io::Read;


fn main() {
    let html = get_html("渋谷駅", "ラーメン");
    let url_list = get_url_from(html.as_str());
    for url in url_list {
        println!("{}", url);
    }
}

fn get_html(location: &str, token: &str) -> String {

    let url = format!("https://tabelog.com/tokyo/A1303/A130301/R4698/rstLst/?vs=1&sa={}&sk={}&lid=hd_search1&vac_net=&svd=20180203&svt=1900&svps=2&hfc=1&Cat=MC&sw=", location, token);

    let mut response = String::new();
    HttpsClientBuilder::build()
        .get(&url)
        .send()
        .unwrap()
        .read_to_string(&mut response)
        .unwrap();

    response
}

fn get_url_from(html: &str) -> Vec<String> {

    let re = Regex::new("data-detail-url=\"(?P<url>.*)\" data-rst-id").unwrap();

    re.captures_iter(html).map(|capture| {
        capture["url"].to_string()
    }).collect()
}
