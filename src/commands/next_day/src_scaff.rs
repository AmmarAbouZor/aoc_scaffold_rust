use anyhow::Result;
use lazy_static::lazy_static;
use std::{fs, path::PathBuf};

pub fn scaff_next_day(current_dir: &PathBuf, year: &str) -> Result<u8> {
    let src_path = current_dir.join("src");

    let year_file = src_path.join(get_year_file_name(year));
    if year_file.exists() {
        append_next_day(&src_path, year)
    } else {
        create_year(&src_path, year)
    }
}

fn get_year_dir_name(year: &str) -> String {
    format!("year_{year}")
}

fn get_year_file_name(year: &str) -> String {
    format!("{}.rs", get_year_dir_name(year))
}

fn append_next_day(src_path: &PathBuf, year: &str) -> Result<u8> {
    todo!()
}

fn create_year(src_path: &PathBuf, year: &str) -> Result<u8> {
    println!("year '{year}' doesn't exist yet. creating year '{year}'...");

    let year_dir_path = src_path.join(get_year_dir_name(year));
    fs::create_dir(year_dir_path.clone())?;

    generate_day_file(&year_dir_path, 1)?;
    generate_year_file(src_path, year)?;
    // TODO manipulate main
    Ok(1)
}

fn get_day_name(day: u8) -> String {
    format!("day_{:02}", day)
}

fn generate_day_file(dir_path: &PathBuf, day: u8) -> Result<()> {
    lazy_static! {
        static ref DAY_TEMPLATE: &'static str = r"fn part_1() {}
fn part_2() {}

pub fn run() {
    part_1();
    part_2();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {}
}";
    }

    let file_path = dir_path.join(get_day_name(day)).with_extension("rs");
    fs::write(file_path, DAY_TEMPLATE.clone())?;

    Ok(())
}

fn get_year_name(year: &str) -> String {
    format!("year_{year}")
}

fn generate_year_file(dir_path: &PathBuf, year: &str) -> Result<()> {
    lazy_static! {
        static ref YEAR_TEMPLATE: &'static str = r#"mod day_01;

pub fn run() {
    run_day(1);
}

fn run_day(day: u8) {
    match day {
        1 => day_01::run(),
        _ => unreachable!("day not implemented"),
    }
}
"#;
    }

    let file_path = dir_path.join(get_year_name(year)).with_extension("rs");
    fs::write(file_path, YEAR_TEMPLATE.clone())?;
    Ok(())
}
