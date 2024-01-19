use chrono::{Local, NaiveDate};
use curl::easy::Easy;
use encoding::all::ISO_8859_2;
use encoding::{DecoderTrap, Encoding};
use regex::{self, Regex};
use std::collections::BTreeMap;

static SEP: &str = ";";
static WEEK_DAYS: [&str; 7] = [
    "hétfő",
    "kedd",
    "szerda",
    "csütörtök",
    "péntek",
    "szombat",
    "vasárnap",
];

pub(crate) fn scrape() {
    let mut web_page = String::new();
    let mut handle = Easy::new();
    handle
        .url("https://www.minerofil.hu/asvanyborzenaptar.php")
        .unwrap();
    let mut transer = handle.transfer();
    transer
        .write_function(|data| {
            let bytes = ISO_8859_2.decode(data, DecoderTrap::Ignore).unwrap();
            web_page.push_str(&bytes);
            Ok(data.len())
        })
        .unwrap();
    transer.perform().unwrap();
    drop(transer);
    let re_idopont = Regex::new(r"Időpontja.*(2024.*)\.<").unwrap();
    let re_varos = Regex::new(r"Város.*bgcolor.*>(.*)<\/td").unwrap();
    let re_helyszine = Regex::new(r"Börze helyszíne.*bgcolor.*>(.*)<\/td").unwrap();
    let re_neve = Regex::new(r"Börze neve.*bgcolor.*>(.*)<\/td").unwrap();
    let re_honlapja = Regex::new("Szervező honlapja.*href=\"(.*)\">").unwrap();
    let mut city = String::new();
    let mut name = String::new();
    for line in web_page.lines() {
        process_idopont(line, &re_idopont);
        process_location(
            line,
            &mut city,
            &mut name,
            &re_varos,
            &re_neve,
            &re_helyszine,
        );
        process_honlapja(line, &re_honlapja);
    }
}

fn process_idopont(line: &str, re_idopont: &Regex) {
    let months = BTreeMap::from([
        ("január ", "01."),
        ("február ", "02."),
        ("március ", "03."),
        ("április ", "04."),
        ("május ", "05."),
        ("június ", "06."),
        ("július ", "07."),
        ("augusztus ", "08."),
        ("szeptember ", "09."),
        ("október ", "10."),
        ("november ", "11."),
        ("december ", "12."),
    ]);
    if let Some(idopont) = re_idopont.captures(line) {
        let idopont = idopont.get(1).unwrap().as_str();
        let mut date = idopont.to_string();
        for e in months.iter() {
            date = date.replace(e.0, e.1);
        }
        date = date.replace(' ', "");
        let mut it = date.split('.');
        let year_month = it.next().unwrap().to_owned() + "." + it.next().unwrap() + ".";
        let mut day = it.next().unwrap().split('-').next().unwrap().to_string();
        if day.len() == 1 {
            day = "0".to_owned() + day.as_str();
        }
        date = year_month + day.as_str();
        if let Ok(dt) = NaiveDate::parse_from_str(date.as_str(), "%Y.%m.%d") {
            let now = Local::now();
            let today = now.date_naive();
            if today <= dt {
                let week_day = WEEK_DAYS[dt.format("%u").to_string().parse::<usize>().unwrap() - 1];
                print!("\n{}{}{}{}ásványbörze{}", date, SEP, week_day, SEP, SEP);
            }
        }
    }
}

fn process_location(
    line: &str,
    city: &mut String,
    name: &mut String,
    re_varos: &Regex,
    re_neve: &Regex,
    re_helyszine: &Regex,
) {
    if let Some(varos) = re_varos.captures(line) {
        *city = varos.get(1).unwrap().as_str().to_string();
    }
    if let Some(helyszine) = re_helyszine.captures(line) {
        let location = helyszine.get(1).unwrap().as_str();
        print!("{} {}{}{}{}", city, location, SEP, name, SEP);
    }
    if let Some(neve) = re_neve.captures(line) {
        let neve = neve.get(1).unwrap().as_str();
        *name = neve.to_string();
        *name = name.replace("&#34;", "\"");
        *name = name.replace("&amp;", "&");
    }
}

fn process_honlapja(line: &str, re_honlapja: &Regex) {
    let honlapja = re_honlapja.captures(line);
    if honlapja.is_some() {
        let honlapja = honlapja.unwrap().get(1).unwrap().as_str();
        let mut homepage = honlapja.to_string();
        homepage = homepage.replace("http://https://", "https://");
        print!("{}", homepage);
    }
}
