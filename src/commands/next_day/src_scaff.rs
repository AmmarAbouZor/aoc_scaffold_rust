use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::commands;

pub fn scaff_next_day(current_dir: &Path, year: &str) -> Result<u8> {
    let src_path = current_dir.join("src");

    let year_file = src_path.join(get_year_file_name(year));
    if year_file.exists() {
        append_next_day(&src_path, year)
    } else {
        create_year(&src_path, year)
    }
}

fn get_year_file_name(year: &str) -> String {
    format!("{}.rs", commands::get_year_name(year))
}

fn create_year(src_path: &Path, year: &str) -> Result<u8> {
    println!("year '{year}' doesn't exist yet. creating year '{year}'...");

    let year_dir_path = src_path.join(commands::get_year_name(year));
    fs::create_dir(year_dir_path.clone())?;

    generate_day_file(&year_dir_path, year, 1)?;
    generate_year_file(src_path, year)?;
    insert_year_main(src_path, year)?;
    Ok(1)
}

fn generate_day_file(year_dir_path: &Path, year: &str, day: u8) -> Result<()> {
    let day_template: String = format!(
        "use crate::utls::read_text_from_file;

fn part_1(input: &'static str) {{}}

fn part_2(input: &'static str) {{}}

pub fn run() {{
    let input = read_text_from_file(\"{year}\", \"{day:02}\").leak();
    part_1(input);
    part_2(input);
}}

#[cfg(test)]
mod test {{
    use super::*;

    const INPUT: &str = \"\";

    #[test]
    fn test_solution() {{}}
}}"
    );

    let file_path = year_dir_path
        .join(commands::get_day_name(day))
        .with_extension("rs");
    fs::write(&file_path, day_template.as_str())?;

    println!("File '{}' has been created", file_path.to_string_lossy());

    Ok(())
}

fn generate_year_file(dir_path: &Path, year: &str) -> Result<()> {
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

    let file_path = dir_path
        .join(commands::get_year_name(year))
        .with_extension("rs");
    fs::write(file_path, *YEAR_TEMPLATE)?;
    Ok(())
}

fn insert_year_main(dir_path: &Path, year: &str) -> Result<()> {
    let main_path = dir_path.join("main.rs");
    let mut text = fs::read_to_string(main_path.clone())?;

    // insert mod
    let insert_mod_index = {
        if let Some(last_entry) = text.rfind("mod year_") {
            let mut insert_index = text[last_entry..]
                .find(';')
                .context("main content is changed outside of this program")?;

            insert_index += last_entry;

            text.insert(insert_index + 1, '\n');

            insert_index + 2
        } else {
            text.insert_str(0, "\n\n");
            0
        }
    };

    let year_mod = format!("mod {};", commands::get_year_name(year));

    text.insert_str(insert_mod_index, &year_mod);

    // insert in main and run_year functions
    let run_func_with_arg = format!("run_year(\"{year}\");");
    let year_match = format!(
        "\"{year}\" => {}::run(),\n        ",
        commands::get_year_name(year)
    );
    if text.contains("run_year(\"") {
        let run_year_regex = get_run_year_regex();

        text = run_year_regex.replace(&text, run_func_with_arg).into();

        // year match
        let insert_index_match = text
            .find(r#"_ => unreachable!("year not implemented")"#)
            .context("main content is changed outside of this program")?;

        text.insert_str(insert_index_match, &year_match);
    } else {
        let search_parr_main = r" main() {";
        let main_index = text
            .find(search_parr_main)
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

fn get_run_year_regex() -> &'static Regex {
    lazy_static! {
        static ref YEAR_ARG_REGEX: Regex = Regex::new(r#"run_year\(".+"\);"#).unwrap();
    }

    &YEAR_ARG_REGEX
}

fn append_next_day(src_path: &Path, year: &str) -> Result<u8> {
    update_year_main(src_path, year)?;

    let year_dir_path = src_path.join(commands::get_year_name(year));
    let last_day_num = commands::get_last_day(&year_dir_path)?;
    let next_day = last_day_num + 1;
    generate_day_file(&year_dir_path, year, next_day)?;

    let year_file_path = year_dir_path.with_extension("rs");
    insert_day_in_year(&year_file_path, next_day)?;
    Ok(next_day)
}

fn update_year_main(dir_path: &Path, year: &str) -> Result<()> {
    let main_path = dir_path.join("main.rs");
    let mut text = fs::read_to_string(main_path.clone())?;

    // update run_year call in run functions
    let run_func_with_arg = format!("run_year(\"{year}\");");
    if text.contains(&run_func_with_arg) {
        return Ok(());
    }

    if text.contains("run_year(\"") {
        let run_year_regex = get_run_year_regex();
        text = run_year_regex.replace(&text, run_func_with_arg).into();
    } else {
        bail!("main content is changed outside of this program, run_year() fn can't be found");
    }

    fs::write(main_path, text)?;
    Ok(())
}

fn insert_day_in_year(year_file_path: &PathBuf, day: u8) -> Result<()> {
    let mut text = fs::read_to_string(year_file_path.clone())?;

    // insert mod
    let insert_mod_index = {
        if let Some(last_entry) = text.rfind("mod day_") {
            let mut insert_index = text[last_entry..]
                .find(';')
                .context("year content is changed outside of this program")?;

            insert_index += last_entry;

            text.insert(insert_index + 1, '\n');

            insert_index + 2
        } else {
            text.insert_str(0, "\n\n");
            0
        }
    };

    let day_mod = format!("mod {};", commands::get_day_name(day));

    text.insert_str(insert_mod_index, &day_mod);

    // insert in run functions
    if text.contains("run_day(") {
        lazy_static! {
            static ref DAY_FUNC_REGEX: Regex = Regex::new(r"run_day\(.+\);").unwrap();
        }

        let run_func_with_arg = format!("run_day({day});");
        text = DAY_FUNC_REGEX.replace(&text, run_func_with_arg).into();

        // day match
        let insert_index_match = text
            .find(r#"_ => unreachable!("day not implemented")"#)
            .context("year file content is changed outside of this program")?;

        let day_match = format!("{day} => {}::run(),\n        ", commands::get_day_name(day));
        text.insert_str(insert_index_match, &day_match);
    } else {
        bail!("year file content is changed outside of this program")
    }

    fs::write(year_file_path, text)?;
    Ok(())
}
