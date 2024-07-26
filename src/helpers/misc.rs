use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use rand::seq::SliceRandom;
use rand;
use rand::Rng;
use ordinal::Ordinal;

pub fn random_choice<'a, T>(items: &'a [T]) -> Option<&'a T> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)
}

pub fn random_int(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower_bound..=upper_bound)
    
}

pub fn random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let mut rng = rand::thread_rng();

    let days = (end - start).num_days();
    let random_days = rng.gen_range(0..=days); 
    
    start + Duration::days(random_days)
}

pub fn pretty_date(date: &NaiveDate) -> String {
    let day = date.day();
    let day_ordinal = Ordinal(day).to_string();  // could error handle but lazy
    let month = date.format("%B");
    let year = date.year();

    format!("{} {} {}", month, day_ordinal, year)
}

pub fn pretty_datetime(datetime: &NaiveDateTime) -> String {
    let time = datetime.format("%H:%M:%S");

    format!("{} - {}", time, pretty_date(&datetime.date()))
}








