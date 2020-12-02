use calamine::{open_workbook_auto, DataType, Reader, Sheets};
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
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");

    // Now get all formula!
    // let sheets = workbook.sheet_names().to_owned();
    // for s in sheets {
    //     range(&s, &mut workbook);
    // }

    range("广灵一斗泉一期", &mut workbook);
}

fn range(sheet: &str, workbook: &mut Sheets) {
    // Read whole worksheet data and provide some statistics
    if let Some(Ok(range)) = workbook.worksheet_range(sheet) {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} (rows={},cols={}) cells in '{}', including {} non empty cells",
            total_cells,
            range.height(),
            range.width(),
            sheet,
            non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );

        for i in 0..range.width() {
            println!("{} {:?}", i, range.get_value((1, i as u32)));
        }
    }
}
