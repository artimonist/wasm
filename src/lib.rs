mod compress;
mod console;
mod encrypt;
mod generator;

pub(crate) use console::console_log as log;

pub use compress::compress;
pub use encrypt::encrypt;
pub use generator::{complex_init, generate, simple_init};
