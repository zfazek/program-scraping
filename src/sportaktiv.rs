use chrono::{Local, NaiveDate};
use curl::easy::Easy;
use regex::{self, Regex};
use std::collections::BTreeMap;

use crate::SEP;
use crate::WEEK_DAYS;

pub(crate) fn scrape() {
    let mut web_page = String::new();
    let mut handle = Easy::new();
    handle.url("https://sportaktiv.hu/fooldal/").unwrap();
    let mut transer = handle.transfer();
    transer
        .write_function(|data| {
            web_page.push_str(std::str::from_utf8(data).unwrap());
            Ok(data.len())
        })
        .unwrap();
    transer.perform().unwrap();
    drop(transer);
    let web_page = include_str!("../sportaktiv_hu_index.html");
    let re = Regex::new(r#"Időpont: (\d+)\. (.*) (\d+)\..*túráról:&nbsp;(.*)<"#).unwrap();
    let re_link = Regex::new(r#"href="(.*)" class.*bővebben"#).unwrap();
    for line in web_page.lines() {
        process_line(line, &re);
        process_link(line, &re_link);
    }
}

fn process_line(line: &str, re: &Regex) {
    let months = BTreeMap::from([
        ("január", "01."),
        ("február", "02."),
        ("március", "03."),
        ("április", "04."),
        ("május", "05."),
        ("június", "06."),
        ("július", "07."),
        ("augusztus", "08."),
        ("szeptember", "09."),
        ("október", "10."),
        ("november", "11."),
        ("december", "12."),
    ]);
    if let Some(cap) = re.captures(line) {
        let y = cap.get(1).unwrap().as_str();
        let m = cap.get(2).unwrap().as_str();
        let mut d = cap.get(3).unwrap().as_str().to_string();
        if d.len() == 1 {
            d = "0".to_owned() + d.as_str();
        }
        let t = cap.get(4).unwrap().as_str();
        let now = Local::now();
        let mut date = y.to_owned() + "." + m + d.as_str();
        for e in months.iter() {
            date = date.replace(e.0, e.1);
        }
        if let Ok(dt) = NaiveDate::parse_from_str(date.as_str(), "%Y.%m.%d") {
            let today = now.date_naive();
            if today < dt {
                let week_day = WEEK_DAYS[dt.format("%u").to_string().parse::<usize>().unwrap() - 1];
                print!(
                    "{}{}{}{}kerékpár{}Balatonfüred{}{}{}",
                    date, SEP, week_day, SEP, SEP, SEP, t, SEP
                );
            }
        }
    }
}

fn process_link(line: &str, re_link: &Regex) {
    if let Some(cap) = re_link.captures(line) {
        let link = cap.get(1).unwrap().as_str().to_string();
        println!("{link}");
    }
}
