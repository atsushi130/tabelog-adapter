
use tabelog::search_condition::SearchCondition;
use https_client::HttpsClientBuilder;
use std::io::Read;

pub struct TabelogSource;

impl<'a> TabelogSource {

    const BASE_URL: &'a str   = "https://tabelog.com/";
    const PARAMETERS: &'a str = "lid=hd_search1&vac_net=&svd=20180203&svt=1900&svps=2&hfc=1&Cat=MC&sw=";

    pub fn get_source(&self, search_condition: &SearchCondition) -> String {

        let url = self.build_url(search_condition);

        let mut response = String::new();
        HttpsClientBuilder::build()
            .get(&url)
            .send()
            .unwrap()
            .read_to_string(&mut response)
            .unwrap();

        response
    }

    fn build_url(&self, search_condition: &SearchCondition) -> String {
        format!("{}tokyo/A1303/A130301/R4698/rstLst/?vs=1&sa={}&sk={}&{}",
                TabelogSource::BASE_URL,
                search_condition.location,
                search_condition.word,
                TabelogSource::PARAMETERS
        )
    }
}