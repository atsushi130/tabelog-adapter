
// Copyright (c) 2018 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;

pub struct HttpsClientBuilder;

impl HttpsClientBuilder {

    pub fn build() -> Client {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    }
}