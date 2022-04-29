use std::path::Path;

pub fn get_dirent_count_by_names<I, F, P>(names: I, filter: F) -> usize
where
    P: AsRef<Path>,
    I: Iterator<Item = P>,
    F: Fn(&Path) -> bool,
{
    names.filter(|name| filter(name.as_ref())).count()
}
