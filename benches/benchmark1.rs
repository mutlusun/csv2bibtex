use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv2bibtex::bibwriter;
use csv2bibtex::bibwriter::BibWrite;
use csv2bibtex::converter;
use csv2bibtex::csvreader;
use csv2bibtex::entry;

fn bench_with_zero_fields(input: &str) {
    // create hasmap
    let mut csv_field_mapping = std::collections::HashMap::new();

    // create new csvparser, converter, and writer
    let output = std::vec::Vec::new();
    let csvparser = csvreader::Reader::new(input.as_bytes(), ",", false);
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);
    let mut writer = bibwriter::BiblatexWriter::new(output);

    // main loop
    for entry in csvparser {
        let entry = entry.unwrap();
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry).unwrap();
    }
}

// five fields that occure in the file / input
fn bench_with_five_valid_fields(input: &str) {
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

    // create new csvparser, converter, and writer
    let output = std::vec::Vec::new();
    let csvparser = csvreader::Reader::new(input.as_bytes(), ",", false);
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);
    let mut writer = bibwriter::BiblatexWriter::new(output);

    // main loop
    for entry in csvparser {
        let entry = entry.unwrap();
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry).unwrap();
    }
}

// five fields that do not occure in the file / input
fn bench_with_five_invalid_fields(input: &str) {
    // build field hash map
    let mut csv_field_mapping = std::collections::HashMap::new();
    csv_field_mapping.insert(String::from("title"), String::from("[[TITLE]]"));
    csv_field_mapping.insert(String::from("author"), String::from("[[AUTHOR]]"));
    csv_field_mapping.insert(String::from("journal"), String::from("[[JOURNAL]]"));
    csv_field_mapping.insert(String::from("year"), String::from("[[YEAR]]"));
    csv_field_mapping.insert(String::from("volume"), String::from("[[vOLUME]]"));

    // create new csvparser, converter, and writer
    let output = std::vec::Vec::new();
    let csvparser = csvreader::Reader::new(input.as_bytes(), ",", false);
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);
    let mut writer = bibwriter::BiblatexWriter::new(output);

    // main loop
    for entry in csvparser {
        let entry = entry.unwrap();
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry).unwrap();
    }
}

// ten fields that occure in the file / input
fn bench_with_ten_valid_fields(input: &str) {
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

    // create new csvparser, converter, and writer
    let output = std::vec::Vec::new();
    let csvparser = csvreader::Reader::new(input.as_bytes(), ",", false);
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);
    let mut writer = bibwriter::BiblatexWriter::new(output);

    // main loop
    for entry in csvparser {
        let entry = entry.unwrap();
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry).unwrap();
    }
}

// ten fields that do not occure in the file / input
fn bench_with_ten_invalid_fields(input: &str) {
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

    // create new csvparser, converter, and writer
    let output = std::vec::Vec::new();
    let csvparser = csvreader::Reader::new(input.as_bytes(), ",", false);
    let converter = converter::FieldConverter::new(&mut csv_field_mapping, None);
    let mut writer = bibwriter::BiblatexWriter::new(output);

    // main loop
    for entry in csvparser {
        let entry = entry.unwrap();
        let entry = converter.convert_fields(entry);
        let entry = entry::Entry::from_hashmap(entry);
        writer.write(&entry).unwrap();
    }
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
