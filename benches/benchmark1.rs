use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn run_main_loop(config: &mut csv2bibtex::args::Config) {
    // that's strange, does not work by reference
    let mut config = config.clone();

    // run main function
    csv2bibtex::run(&mut config).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    // build config structure
    let mut config = csv2bibtex::args::Config::default();
    config.file_input = std::path::PathBuf::from("./benches/benchmark1-input1.csv");
    config.file_output = std::path::PathBuf::from("./benches/tmp-benchmark1-output1.bib");
    config.csv_delimiter = String::from("\t");
    config.csv_lazy = true;

    // benchmark runs (100 lines, 0 fields)
    c.bench_function("0 fields, 100 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five fields and run again
    config
        .csv_field_mapping
        .insert(String::from("entrytype"), String::from("article"));
    config
        .csv_field_mapping
        .insert(String::from("author"), String::from("[[AU]]"));
    config
        .csv_field_mapping
        .insert(String::from("title"), String::from("[[TI]]"));
    config
        .csv_field_mapping
        .insert(String::from("journal"), String::from("[[SO]]"));
    config
        .csv_field_mapping
        .insert(String::from("volume"), String::from("[[VL]]"));
    c.bench_function("5 valid fields, 100 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five fields and run again
    config
        .csv_field_mapping
        .insert(String::from("number"), String::from("[[IS]]"));
    config
        .csv_field_mapping
        .insert(String::from("pages"), String::from("[[BP]]--[[EP]]"));
    config
        .csv_field_mapping
        .insert(String::from("doi"), String::from("[[DI]]"));
    config
        .csv_field_mapping
        .insert(String::from("year"), String::from("[[PY]]"));
    config
        .csv_field_mapping
        .insert(String::from("abstract"), String::from("[[AB]]"));
    c.bench_function("10 valid fields, 100 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // clear fields, add five invalid fields and run again
    config.csv_field_mapping.clear();
    config
        .csv_field_mapping
        .insert(String::from("entrytype"), String::from("[[article]]"));
    config
        .csv_field_mapping
        .insert(String::from("author"), String::from("[[authors]]"));
    config
        .csv_field_mapping
        .insert(String::from("title"), String::from("[[titles]]"));
    config
        .csv_field_mapping
        .insert(String::from("journal"), String::from("[[journals]]"));
    config
        .csv_field_mapping
        .insert(String::from("volume"), String::from("[[volumes]]"));
    c.bench_function("5 invalid fields, 100 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five invalid fields and run again
    config
        .csv_field_mapping
        .insert(String::from("number"), String::from("[[numbers]]"));
    config
        .csv_field_mapping
        .insert(String::from("pages"), String::from("[[pages]]"));
    config
        .csv_field_mapping
        .insert(String::from("doi"), String::from("[[dois]]"));
    config
        .csv_field_mapping
        .insert(String::from("year"), String::from("[[years]]"));
    config
        .csv_field_mapping
        .insert(String::from("abstract"), String::from("[[abstracts]]"));
    c.bench_function("10 invalid fields, 100 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // switch to 1000 lines input file, clear fields, run again
    config.file_input = std::path::PathBuf::from("./benches/benchmark1-input2.csv");
    config.csv_field_mapping.clear();
    c.bench_function("0 fields, 1000 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five fields and run again
    config
        .csv_field_mapping
        .insert(String::from("entrytype"), String::from("article"));
    config
        .csv_field_mapping
        .insert(String::from("author"), String::from("[[AU]]"));
    config
        .csv_field_mapping
        .insert(String::from("title"), String::from("[[TI]]"));
    config
        .csv_field_mapping
        .insert(String::from("journal"), String::from("[[SO]]"));
    config
        .csv_field_mapping
        .insert(String::from("volume"), String::from("[[VL]]"));
    c.bench_function("5 valid fields, 1000 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five fields and run again
    config
        .csv_field_mapping
        .insert(String::from("number"), String::from("[[IS]]"));
    config
        .csv_field_mapping
        .insert(String::from("pages"), String::from("[[BP]]--[[EP]]"));
    config
        .csv_field_mapping
        .insert(String::from("doi"), String::from("[[DI]]"));
    config
        .csv_field_mapping
        .insert(String::from("year"), String::from("[[PY]]"));
    config
        .csv_field_mapping
        .insert(String::from("abstract"), String::from("[[AB]]"));
    c.bench_function("10 valid fields, 1000 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // clear fields, add five invalid fields and run again
    config.csv_field_mapping.clear();
    config
        .csv_field_mapping
        .insert(String::from("entrytype"), String::from("[[article]]"));
    config
        .csv_field_mapping
        .insert(String::from("author"), String::from("[[authors]]"));
    config
        .csv_field_mapping
        .insert(String::from("title"), String::from("[[titles]]"));
    config
        .csv_field_mapping
        .insert(String::from("journal"), String::from("[[journals]]"));
    config
        .csv_field_mapping
        .insert(String::from("volume"), String::from("[[volumes]]"));
    c.bench_function("5 invalid fields, 1000 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // add five invalid fields and run again
    config
        .csv_field_mapping
        .insert(String::from("number"), String::from("[[numbers]]"));
    config
        .csv_field_mapping
        .insert(String::from("pages"), String::from("[[pages]]"));
    config
        .csv_field_mapping
        .insert(String::from("doi"), String::from("[[dois]]"));
    config
        .csv_field_mapping
        .insert(String::from("year"), String::from("[[years]]"));
    config
        .csv_field_mapping
        .insert(String::from("abstract"), String::from("[[abstracts]]"));
    c.bench_function("10 invalid fields, 1000 lines", |b| {
        b.iter(|| run_main_loop(black_box(&mut config)))
    });

    // clean up
    std::fs::remove_file("./tests/tmp-benchmark1-output1.bib").unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
