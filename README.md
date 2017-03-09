# `ocr_latin_vocabulary`

[![crates.io](https://img.shields.io/crates/v/ocr_latin_vocabulary.svg)](https://crates.io/crates/ocr_latin_vocabulary)
[![Travis CI](https://img.shields.io/travis/saleemrashid/ocr_latin_vocabulary-rs.svg)](https://travis-ci.org/saleemrashid/ocr_latin_vocabulary-rs)

Convert OCR Latin GCSE Defined vocabulary lists to a simple format

## Input Format

Download the OCR Latin GCSE Defined vocabulary lists in Excel format

Then convert to [Comma-separated values](https://en.wikipedia.org/wiki/Comma-separated_values)

### LibreOffice

```bash
libreoffice --headless --convert-to csv --infilter=CSV:44,34,UTF8 FILE
```

`--infilter=CSV:44,34,UTF8` is necessary because LibreOffice defaults to ISO-8859-1

### [unoconv](https://github.com/dagwieers/unoconv)

```bash
unoconv -f csv FILE
```

## Output Format

The simple format can be imported into some vocabulary testing software

```text
[Latin descriptor]
[English translation]/[English translation]/[English translation]
```

# License

`ocr_latin_vocabulary` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
