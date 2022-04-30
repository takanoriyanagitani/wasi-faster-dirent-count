use std::fs::{read_dir, DirEntry};
use std::path::Path;

fn iter2count<I, F>(items: I, filter: &F) -> usize
where
    I: Iterator<Item = DirEntry>,
    F: Fn(&Path) -> bool,
{
    items
        .map(|dirent| dirent.path())
        .filter(|p| filter(p.as_path()))
        .count()
}

fn month2cnt<P, F>(month: P, filter: &F) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
{
    read_dir(month)
        .ok()
        .map(|i| i.flat_map(|rslt| rslt.ok()))
        .map(|i| iter2count(i, filter))
        .unwrap_or_default()
}

fn new_dir_cnt<P, F>(filter: F) -> impl Fn(P) -> usize
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
{
    move |p: P| month2cnt(p, &filter)
}

fn new_simple_filter() -> impl Fn(&Path) -> bool {
    move |p: &Path| p.exists()
}

fn get_year_path() -> String {
    std::env::var("ENV_YEAR_DIR").unwrap_or_else(|_| String::from("./testdata/root.d/data.d/2022"))
}

fn main() {
    let ypath = get_year_path();
    let filter = new_simple_filter();
    let counter = new_dir_cnt(filter);
    let cnt: usize = counter(ypath);
    println!("{}", cnt);
}
