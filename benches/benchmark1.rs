use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv2bibtex::converter;
use csv2bibtex::csvparser;
use csv2bibtex::entry;

fn bench_with_zero_fields(input: &str) -> Vec<String> {
    // create hasmap
    let mut csv_field_mapping = std::collections::HashMap::new();

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(input.as_bytes(), ",");
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);

    // main loop
    let mut ret: Vec<String> = vec![String::from(""); 0];
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        ret.push(entry.to_biblatex_string());
    }

    ret
}

// five fields that occure in the file / input
fn bench_with_five_valid_fields(input: &str) -> Vec<String> {
    // build field hash map
    let mut csv_field_mapping = std::collections::HashMap::new();
    csv_field_mapping.insert(String::from("title"), String::from("[[Document Title]]"));
    csv_field_mapping.insert(String::from("author"), String::from("[[Authors]]"));
    csv_field_mapping.insert(
        String::from("journal"),
        String::from("[[Publication Title]]"),
    );
    csv_field_mapping.insert(String::from("year"), String::from("[[Publication Year]]"));
    csv_field_mapping.insert(String::from("volume"), String::from("[[Volume]]"));

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(input.as_bytes(), ",");
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None).add_defaults();

    // main loop
    let mut ret: Vec<String> = vec![String::from(""); 0];
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        ret.push(entry.to_biblatex_string());
    }

    ret
}

// five fields that do not occure in the file / input
fn bench_with_five_invalid_fields(input: &str) -> Vec<String> {
    // build field hash map
    let mut csv_field_mapping = std::collections::HashMap::new();
    csv_field_mapping.insert(String::from("title"), String::from("[[TITLE]]"));
    csv_field_mapping.insert(String::from("author"), String::from("[[AUTHOR]]"));
    csv_field_mapping.insert(String::from("journal"), String::from("[[JOURNAL]]"));
    csv_field_mapping.insert(String::from("year"), String::from("[[YEAR]]"));
    csv_field_mapping.insert(String::from("volume"), String::from("[[vOLUME]]"));

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(input.as_bytes(), ",");
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None).add_defaults();

    // main loop
    let mut ret: Vec<String> = vec![String::from(""); 0];
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        ret.push(entry.to_biblatex_string());
    }

    ret
}

// ten fields that occure in the file / input
fn bench_with_ten_valid_fields(input: &str) -> Vec<String> {
    // build field hash map
    let mut csv_field_mapping = std::collections::HashMap::new();
    csv_field_mapping.insert(String::from("title"), String::from("[[Document Title]]"));
    csv_field_mapping.insert(String::from("author"), String::from("[[Authors]]"));
    csv_field_mapping.insert(
        String::from("journal"),
        String::from("[[Publication Title]]"),
    );
    csv_field_mapping.insert(String::from("year"), String::from("[[Publication Year]]"));
    csv_field_mapping.insert(String::from("volume"), String::from("[[Volume]]"));
    csv_field_mapping.insert(String::from("number"), String::from("[[Issue]]"));
    csv_field_mapping.insert(
        String::from("pages"),
        String::from("[[Start Page]]--[[End Page]]"),
    );
    csv_field_mapping.insert(String::from("abstract"), String::from("[[Abstract]]"));
    csv_field_mapping.insert(String::from("issn"), String::from("[[ISSN]]"));
    csv_field_mapping.insert(String::from("isbn"), String::from("[[ISBNs]]"));

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(input.as_bytes(), ",");
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None).add_defaults();

    // main loop
    let mut ret: Vec<String> = vec![String::from(""); 0];
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        ret.push(entry.to_biblatex_string());
    }

    ret
}

// ten fields that do not occure in the file / input
fn bench_with_ten_invalid_fields(input: &str) -> Vec<String> {
    // build field hash map
    let mut csv_field_mapping = std::collections::HashMap::new();
    csv_field_mapping.insert(String::from("title"), String::from("[[TITLE]]"));
    csv_field_mapping.insert(String::from("author"), String::from("[[AUTHOR]]"));
    csv_field_mapping.insert(String::from("journal"), String::from("[[JOURNAL]]"));
    csv_field_mapping.insert(String::from("year"), String::from("[[YEAR]]"));
    csv_field_mapping.insert(String::from("volume"), String::from("[[vOLUME]]"));
    csv_field_mapping.insert(String::from("number"), String::from("[[NUMBER]]"));
    csv_field_mapping.insert(String::from("pages"), String::from("[[PAGES]]"));
    csv_field_mapping.insert(String::from("abstract"), String::from("[[aBSTRACT]]"));
    csv_field_mapping.insert(String::from("issn"), String::from("[[ISSNaa]]"));
    csv_field_mapping.insert(String::from("isbn"), String::from("[[ISBNSaa]]"));

    // create new csvparser, converter
    let csvparser = csvparser::Parser::new(input.as_bytes(), ",");
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None).add_defaults();

    // main loop
    let mut ret: Vec<String> = vec![String::from(""); 0];
    for entry in csvparser {
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        ret.push(entry.to_biblatex_string());
    }

    ret
}

fn criterion_benchmark(c: &mut Criterion) {
    // create a data set
    let input = std::fs::read_to_string("./tests/test1-input1.csv").unwrap();
    let input1e2 = (0..25).map(|_| input.clone()).collect::<Vec<_>>().concat();
    let input1e3 = (0..10)
        .map(|_| input1e2.clone())
        .collect::<Vec<_>>()
        .concat();

    c.bench_function("0 fields, 100 lines", |b| {
        b.iter(|| bench_with_zero_fields(black_box(&input1e2)))
    });
    c.bench_function("5 valid fields, 100 lines", |b| {
        b.iter(|| bench_with_five_valid_fields(black_box(&input1e2)))
    });
    c.bench_function("10 valid fields, 100 lines", |b| {
        b.iter(|| bench_with_ten_valid_fields(black_box(&input1e2)))
    });
    c.bench_function("5 invalid fields, 100 lines", |b| {
        b.iter(|| bench_with_five_invalid_fields(black_box(&input1e2)))
    });
    c.bench_function("10 invalid fields, 100 lines", |b| {
        b.iter(|| bench_with_ten_invalid_fields(black_box(&input1e2)))
    });
    c.bench_function("0 fields, 1000 lines", |b| {
        b.iter(|| bench_with_zero_fields(black_box(&input1e3)))
    });
    c.bench_function("5 valid fields, 1000 lines", |b| {
        b.iter(|| bench_with_five_valid_fields(black_box(&input1e3)))
    });
    c.bench_function("10 valid fields, 1000 lines", |b| {
        b.iter(|| bench_with_ten_valid_fields(black_box(&input1e3)))
    });
    c.bench_function("5 invalid fields, 1000 lines", |b| {
        b.iter(|| bench_with_five_invalid_fields(black_box(&input1e3)))
    });
    c.bench_function("10 invalid fields, 1000 lines", |b| {
        b.iter(|| bench_with_ten_invalid_fields(black_box(&input1e3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
