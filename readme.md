# tabelog-searcher
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./license-mit)  
tabelog-searcher's searchable area is Japan.

## Usage
```rust
extern crate tabelog_searcher;
use tabelog_searcher::TabelogClient;

fn main() {

    let result = TabelogClient.search("渋谷", "ラーメン");
    
    match result.is_empty() {
        true  => println!("not found."),
        false =>
            for url in result {
                println!("{}", url);
            }
    }
}
```

## Support area
- Japan

## License
tabelog-searcher is available under the MIT and Apache 2.0 license. See the [LICENSE file](https://github.com/atsushi130/tabelog-searcher/blob/master/license-mit).
