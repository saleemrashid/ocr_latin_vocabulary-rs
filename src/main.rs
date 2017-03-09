extern crate csv;
extern crate unicode_normalization;

use csv::Reader;
use std::io;
use unicode_normalization::UnicodeNormalization;

const REQUIRED_COLUMN_COUNT: usize = 4;

fn main() {
    let mut reader = Reader::from_reader(io::stdin()).has_headers(false);

    let records = reader.byte_records()
        .map(Result::unwrap)
        .inspect(|r| assert_eq!(r.len(), REQUIRED_COLUMN_COUNT))
        // Remove UTF-8 invalid header
        .skip_while(|r| {
            r.iter()
                .rposition(|s| !s.is_empty()) != Some(r.len() - 1)
        })
        // Convert to UTF-8
        .map(|r| {
            r.into_iter()
                .map(|s| String::from_utf8_lossy(&s).trim().nfkd().collect::<String>())
                .collect::<Vec<_>>()
        });

    let words = records.map(|r| {
            let extra_information = &r[1];

            let words = r.first()
                .unwrap()
                .split(',')
                .map(str::trim)
                .map(|s| if extra_information.is_empty() || extra_information == "indeclinable" {
                         s.to_owned()
                     } else if extra_information.starts_with('+') {
                    format!("{} {}", s, extra_information)
                } else {
                    format!("{}, {}", s, extra_information)
                })
                .collect::<Vec<_>>();

            let translations = r[3]
                .split(',')
                .map(str::trim)
                .map(str::to_owned)
                .collect::<Vec<_>>();

            (words, translations)
        })
        .collect::<Vec<_>>();

    for (words, translations) in words {
        println!("{}\n{}", words.join("/"), translations.join("/"));
    }
}
