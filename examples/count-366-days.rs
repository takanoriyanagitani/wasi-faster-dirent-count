use std::path::{Path, PathBuf};

use wasi_faster_dirent_count::date::new_simple_days_counter_for_year;

fn new_path_filter() -> impl Fn(&Path) -> bool {
    |p: &Path| p.exists()
}

fn new_join() -> impl Fn(&Path, u8, u8) -> PathBuf {
    move |p: &Path, mon: u8, day: u8| p.join(format!("{:02}/{:02}", mon, day))
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
	let p = Path::new("./data.d/count-366-days.d/root.d/2020");
	let counter = new_days_counter();
	let c: usize = counter(p);
	println!("{}", c);
}
