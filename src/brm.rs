use chrono::{Local, NaiveDate};
use curl::easy::Easy;
use regex::{self, Regex};
use std::collections::BTreeMap;

use crate::SEP;
use crate::WEEK_DAYS;

pub(crate) fn scrape() {
    let mut web_page = String::new();
    let mut handle = Easy::new();
    handle
        .url("https://www.brmhungary.hu/rendezvenyek/")
        .unwrap();
    let mut transer = handle.transfer();
    transer
        .write_function(|data| {
            web_page.push_str(std::str::from_utf8(data).unwrap());
            Ok(data.len())
        })
        .unwrap();
    transer.perform().unwrap();
    drop(transer);
    let re_month = Regex::new(r"event_dets_all_box_montha.*2024.*>(.*)<").unwrap();
    let re_day = Regex::new(r"event_dets_all_box_daycont.*>([0-9]+)<").unwrap();
    let re_varos = Regex::new("event_link\">([a-zA-Z].*)<").unwrap();
    let re_link_title = Regex::new("event_title.*\"(.*)\">([a-zA-Z].*)</a").unwrap();
    let mut month = String::new();
    let mut link = String::new();
    let mut name = String::new();
    for line in web_page.lines() {
        process_month(line, &mut month, &re_month);
        process_day(line, &re_day, &month);
        process_location(line, &mut link, &mut name, &re_link_title);
        process_varos(line, &link, &name, &re_varos);
    }
}

fn process_month(line: &str, month: &mut String, re_month: &Regex) {
    let months = BTreeMap::from([
        ("január ", "01."),
        ("február ", "02."),
        ("március ", "03."),
        ("ápr", "04."),
        ("máj", "05."),
        ("jún", "06."),
        ("júl", "07."),
        ("aug", "08."),
        ("sze", "09."),
        ("október ", "10."),
        ("november ", "11."),
        ("december ", "12."),
    ]);
    if let Some(m) = re_month.captures(line) {
        let idopont = m.get(1).unwrap().as_str();
        *month = months.get(idopont).unwrap().to_string();
    }
}

fn process_day(line: &str, re_day: &Regex, month: &str) {
    if let Some(d) = re_day.captures(line) {
        let mut day = d.get(1).unwrap().as_str().to_string();
        if day.len() == 1 {
            day = "0".to_owned() + day.as_str();
        }
        let now = Local::now();
        let year = now.format("%Y").to_string();
        let date = year + "." + month + day.as_str();
        if let Ok(dt) = NaiveDate::parse_from_str(date.as_str(), "%Y.%m.%d") {
            let today = now.date_naive();
            if today < dt {
                let week_day = WEEK_DAYS[dt.format("%u").to_string().parse::<usize>().unwrap() - 1];
                print!("\n{}{}{}{}kerékpár{}", date, SEP, week_day, SEP, SEP);
            }
        }
    }
}

fn process_location(line: &str, link: &mut String, name: &mut String, re_link_title: &Regex) {
    if let Some(link_title) = re_link_title.captures(line) {
        *link = "https://www.brmhungary.hu/rendezvenyek".to_owned()
            + link_title.get(1).unwrap().as_str();
        *name = link_title.get(2).unwrap().as_str().to_string();
    }
}

fn process_varos(line: &str, link: &str, name: &str, re_varos: &Regex) {
    if let Some(varos) = re_varos.captures(line) {
        let city = varos.get(1).unwrap().as_str().to_string();
        print!("{}{}{}{}{}{}", city, SEP, name, SEP, link, SEP);
    }
}
