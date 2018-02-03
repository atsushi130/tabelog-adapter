# tabelog-searcher
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit)

## Usage
```rust
extern crate tabelog_searcher;
use tabelog_searcher::TabelogClient;
use tabelog_searcher::SearchCondition;

fn main() {
    let search_condition = &SearchCondition("渋谷", "ラーメン");
    let result = TabelogClient.search(search_condition);
    
    match result.is_empty() {
        true  => println!("not found."),
        false =>
            for url in result {
                println!("{}", url);
            }
    }
}
```

## License
tabelog-searcher is available under the MIT and Apache 2.0 license. See the [LICENSE file](https://github.com/atsushi130/tabelog-searcher/blob/master/license-mit).
