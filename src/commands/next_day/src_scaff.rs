use anyhow::Context;
use anyhow::Result;
use lazy_static::lazy_static;
use regex;
use regex::Regex;
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
    add_year_main(src_path, year)?;
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

fn add_year_main(dir_path: &PathBuf, year: &str) -> Result<()> {
    let main_path = dir_path.join("main.rs");
    let mut text = fs::read_to_string(main_path.clone())?;

    // insert mod
    let insert_mod_index = {
        if let Some(last_entry) = text.rfind("mod year_") {
            let mut insert_index = text[last_entry..]
                .find(";")
                .context("main content is changed outside of this program")?;

            insert_index += last_entry;

            text.insert(insert_index + 1, '\n');

            insert_index + 2
        } else {
            text.insert_str(0, "\n\n");
            0
        }
    };

    let year_mod = format!("mod {};", get_year_name(year));

    text.insert_str(insert_mod_index, &year_mod);

    // insert in main and run_year functions
    let run_func_with_arg = format!("run_year(\"{year}\");");
    let year_match = format!("\"{year}\" => {}::run(),\n        ", get_year_name(year));
    if let Some(_) = text.find("run_year(\"") {
        lazy_static! {
            static ref YEAR_ARG_REGEX: Regex = Regex::new(r#"run_year\("."\);"#).unwrap();
        }

        text = YEAR_ARG_REGEX.replace(&text, run_func_with_arg).into();

        // year match
        let insert_index_match = text
            .find(r#"_ => unreachable!("year not implemented")"#)
            .context("main content is changed outside of this program")?;

        text.insert_str(insert_index_match, &year_match);
    } else {
        let search_parr_main = r" main() {";
        let main_index = text
            .find(&search_parr_main)
            .context("main content is changed outside of this program, main fn can't be found")?;

        let inser_text = format!("\n    {run_func_with_arg}\n");
        text.insert_str(main_index + search_parr_main.len(), &inser_text);
        // create add_year + add it to main
        let run_year_func = format!(
            r#"fn run_year(year: &str) {{
    match year {{
        {year_match}_ => unreachable!("year not implemented"),
    }}
}}"#
        );

        text.push_str(&run_year_func);
    }

    fs::write(main_path, text)?;
    Ok(())
}
