use std::path::{Path, PathBuf};

use crate::core::get_dirent_count_by_names;

pub fn simple_leap_checker(_: i32) -> bool {
    true
}

pub fn simple_day_counter(_: u8, _: bool) -> u8 {
    31
}

pub fn get_days<P, F, J>(month_path: P, filter: &F, join: &J, daymax: u8) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
{
    let days = 0..daymax;
    let names = days.map(|day: u8| join(month_path.as_ref(), day));
    get_dirent_count_by_names(names, filter)
}

pub fn new_days_counter<P, F, J>(daymax: u8) -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
{
    move |month_path: P, filter: &F, join: &J| get_days(month_path, filter, join, daymax)
}

pub fn new_simple_days_counter<P, F, J>() -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
{
    new_days_counter(31)
}

pub fn get_days_for_month<P, F, J, L, C>(
    month_path: P,
    filter: &F,
    join: &J,
    year: i32,
    month: u8,
    leap_check: &L,
    day_counter: &C,
) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
    L: Fn(i32) -> bool,
    C: Fn(u8, bool) -> u8,
{
    let is_leap_year: bool = leap_check(year);
    let daymax: u8 = day_counter(month, is_leap_year);
    get_days(month_path, filter, join, daymax)
}

pub fn new_days_counter_for_month<P, F, J, L, C>(
    year: i32,
    month: u8,
    leap_check: L,
    day_counter: C,
) -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
    L: Fn(i32) -> bool,
    C: Fn(u8, bool) -> u8,
{
    move |month_path: P, filter: &F, join: &J| {
        get_days_for_month(
            month_path,
            filter,
            join,
            year,
            month,
            &leap_check,
            &day_counter,
        )
    }
}

pub fn new_simple_days_counter_for_month<P, F, J>() -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8) -> PathBuf,
{
    new_days_counter_for_month(1970, 1, |_: i32| true, |_: u8, _: bool| 31)
}

pub fn get_days_for_year<P, F, J, L, C>(
    year_path: P,
    filter: &F,
    join: &J,
    year: i32,
    leap_check: &L,
    day_counter: &C,
) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8, u8) -> PathBuf,
    L: Fn(i32) -> bool,
    C: Fn(u8, bool) -> u8,
{
    let month_iter = (0..12).map(|i| i + 1);
    month_iter.fold(0, |tot, month| {
        let is_leap_year: bool = leap_check(year);
        let daymax: u8 = day_counter(month, is_leap_year);
        let days = 0..daymax;
        let names = days.map(|day: u8| join(year_path.as_ref(), month, day));
        tot + get_dirent_count_by_names(names, filter)
    })
}

pub fn new_days_counter_for_year<P, F, J, L, C>(
    year: i32,
    leap_check: L,
    day_counter: C,
) -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8, u8) -> PathBuf,
    L: Fn(i32) -> bool,
    C: Fn(u8, bool) -> u8,
{
    move |year_path: P, filter: &F, join: &J| {
        get_days_for_year(year_path, filter, join, year, &leap_check, &day_counter)
    }
}

pub fn new_simple_days_counter_for_year<P, F, J>() -> impl Fn(P, &F, &J) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
    J: Fn(&Path, u8, u8) -> PathBuf,
{
    new_days_counter_for_year(1970, |_: i32| true, |_: u8, _: bool| 31)
}
