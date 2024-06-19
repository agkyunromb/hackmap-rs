use super::types;

pub mod net;
pub mod ui;

pub use super::D2RVA;

#[allow(unused_imports)]
pub use net as Net;

#[allow(unused_imports)]
pub use ui as UI;

pub fn init(d2client: usize) {
    Net::init(d2client);
    UI::init(d2client);
}
