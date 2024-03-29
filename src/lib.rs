#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tau-OS/libhelium/main/demo/data/libhelium.svg",
    html_favicon_url = "https://raw.githubusercontent.com/tau-OS/libhelium/main/demo/data/libhelium-symbolic.svg"
)]

// Re-export the -sys bindings
pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use gtk;

/// Asserts that this is the main thread and `gtk::init` has been called.
// macro_rules! assert_initialized_main_thread {
//     () => {
//         if !::gtk::is_initialized_main_thread() {
//             if ::gtk::is_initialized() {
//                 panic!("libhelium may only be used from the main thread.");
//             } else {
//                 panic!("Gtk has to be initialized before using libhelium.");
//             }
//         }
//     };
// }

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

mod application;

pub use auto::*;

pub mod builders;
pub mod prelude;
pub mod subclass;
