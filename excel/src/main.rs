use calamine::{open_workbook_auto, DataType, Reader, Sheets};
use std::string::String;
use std::{env, process};

fn main() {
    //let args: Vec<String> = env::args().collect();

    let cfg = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let s = String::from("Hello");
    println!("{:?}", s);

    //println!("{:?}", args);
    println!("{:?}", cfg);

    demo(&cfg);
}

mod config;

fn demo(cfg: &config::Config) {
    // opens a new workbook
    let mut workbook = open_workbook_auto(&cfg.file).expect("Cannot open file");

    // Now get all formula!
    // let sheets = workbook.sheet_names().to_owned();
    // for s in sheets {
    //     range(&s, &mut workbook);
    // }

    range("广灵一斗泉一期", &mut workbook, cfg);
}

fn range(sheet: &str, workbook: &mut Sheets, cfg: &config::Config) {
    // Read whole worksheet data and provide some statistics
    if let Some(Ok(mut range)) = workbook.worksheet_range(sheet) {
        // let total_cells = range.get_size().0 * range.get_size().1;
        // let non_empty_cells: usize = range.used_cells().count();
        // println!(
        //     "Found {} (rows={},cols={}) cells in '{}', including {} non empty cells",
        //     total_cells,
        //     range.height(),
        //     range.width(),
        //     sheet,
        //     non_empty_cells
        // );
        // // alternatively, we can manually filter rows
        // assert_eq!(
        //     non_empty_cells,
        //     range
        //         .rows()
        //         .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
        //         .count()
        // );

        let mut target_index = 0;
        let mut filter_indexs: Vec<usize> = Vec::new();

        for i in 0..range.width() {
            if let Some(subject) = range.get_value((1, i as u32)) {
                if let Some(s) = subject.get_string() {
                    let v: Vec<&str> = s.split('\n').collect();
                    if v.len() == 2 {
                        if v[1] == cfg.target {
                            target_index = i;
                        } else if v[1] == cfg.filter {
                            filter_indexs.push(i);
                        }
                    }
                    println!("{} {:?} {}", i, v, v.len());
                }
            }
        }
        println!("{:?}, {:?}", target_index, filter_indexs);

        for row in 2..range.height() {
            let mut total: f64 = 0.0;
            for col in filter_indexs.iter() {
                if let Some(subject) = range.get_value((row as u32, *col as u32)) {
                    if let Some(f) = subject.get_float() {
                        total += f;
                    }
                }
            }

            range.set_value((row as u32, target_index as u32), DataType::Float(total));
            assert_eq!(
                Some(&DataType::Float(total)),
                range.get_value((row as u32, target_index as u32))
            );
        }
    }
}
