use std::path::{Path, PathBuf};

use wasi_faster_dirent_count::date::new_simple_days_counter_for_year;

fn new_path_filter() -> impl Fn(&Path) -> bool {
    |p: &Path| p.is_file()
}

fn new_join() -> impl Fn(&Path, u8, u8) -> PathBuf {
    move |p: &Path, mon: u8, day: u8| p.join(format!("{:02}-{:02}", mon, day))
}

fn new_days_counter<P>() -> impl Fn(P) -> usize
where
    P: AsRef<Path>,
{
    let counter = new_simple_days_counter_for_year();
    let filter = new_path_filter();
    let join = new_join();
    move |p: P| counter(p, &filter, &join)
}

fn main() {
    let year_root: String = std::env::var("ENV_YEAR_ROOT")
        .unwrap_or_else(|_| String::from("./testdata/root.d/data.d/2022"));
    let counter = new_days_counter();
    let c: usize = counter(PathBuf::from(year_root));
    println!("{}", c);
}
