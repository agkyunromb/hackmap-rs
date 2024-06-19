use super::types;

pub mod stat_list;
pub use stat_list as StatList;

pub fn init(d2common: usize) {
    StatList::init(d2common);
}
