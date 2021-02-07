use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv2bibtex::converter;
use csv2bibtex::csvparser;
use csv2bibtex::entry;

fn bench_with_defaults(input: &str) -> Vec<String> {
    // create hasmap
    let mut csv_field_mapping = std::collections::HashMap::new();

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

fn bench_custom_fields(input: &str) -> Vec<String> {
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
    csv_field_mapping.insert(String::from("doi"), String::from("[[DOI]]"));
    csv_field_mapping.insert(
        String::from("keywords"),
        String::from("[[Author Keywords]]"),
    );

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
    let input1e4 = (0..10)
        .map(|_| input1e3.clone())
        .collect::<Vec<_>>()
        .concat();

    c.bench_function("only defaults (100 lines)", |b| {
        b.iter(|| bench_with_defaults(black_box(&input1e2)))
    });
    c.bench_function("only defaults (1000 lines)", |b| {
        b.iter(|| bench_with_defaults(black_box(&input1e3)))
    });
    c.bench_function("only defaults (10.000 lines)", |b| {
        b.iter(|| bench_with_defaults(black_box(&input1e4)))
    });
    c.bench_function("with custom fields (100 lines)", |b| {
        b.iter(|| bench_custom_fields(black_box(&input1e2)))
    });
    c.bench_function("with custom fields (1000 lines)", |b| {
        b.iter(|| bench_custom_fields(black_box(&input1e3)))
    });
    c.bench_function("with custom fields (10.000 lines)", |b| {
        b.iter(|| bench_custom_fields(black_box(&input1e4)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
