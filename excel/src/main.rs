use calamine::{open_workbook, DataType, Error, RangeDeserializerBuilder, Reader, Xlsx};
use std::{env, process};

fn main() {
    //let args: Vec<String> = env::args().collect();

    let cfg = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("{:?}", args);
    println!("{:?}", cfg);

    demo(&cfg.file);
}

mod config;

fn demo(path: &str) {
    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        println!(
            "found {} formula in '{}'",
            workbook
                .worksheet_formula(&s)
                .expect("sheet not found")
                .expect("error while getting formula")
                .rows()
                .flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                .count(),
            s
        );
    }

    if let Some(Ok(r)) = workbook.worksheet_formula("广灵一斗泉一期") {
        for row in r.rows() {
            println!("row={:?}", row);
        }
    }

    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range("广灵一斗泉一期") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );
    }

    example(path);
}

fn example(path: &str) {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Can not open file");
    let range = workbook
        .worksheet_range("广灵一斗泉一期")
        .expect("Cannot find 'Sheet1'");
    let mut iter = RangeDeserializerBuilder::new()
        .from_range(&range.unwrap())
        .into_iter();

    if let Some(result) = iter.next() {
        let (value, label): (u32, String) = result.enumerate();
        println!("label={}, value={}", label, value);
    }
}
