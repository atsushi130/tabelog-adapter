
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
mod tabelog;

use tabelog::TabelogClient;
use tabelog::SearchCondition;

fn main() {

    let search_condition = &SearchCondition::from("渋谷", "ラーメン");
    let result = TabelogClient.search(search_condition);
    match result.is_empty() {
        true  => println!("お店が見つかりませんでした"),
        false =>
            for url in result {
                println!("{}", url);
            }
    }
}
