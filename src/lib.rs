#![cfg_attr(feature = "dox", feature(doc_cfg))]

// Re-export the -sys bindings
pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use gtk;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("libhelium may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libadwaita.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;

pub use auto::*;

pub mod builders;
pub mod prelude;
