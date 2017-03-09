extern crate csv;

use csv::Reader;
use std::io;

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
                .map(|s| String::from_utf8_lossy(&s).trim().to_owned())
                .collect::<Vec<_>>()
        });

    let words = records.flat_map(|r| {
            let extra_information = &r[1];

            r.first()
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for word in words {
        println!("{}", word);
    }
}
