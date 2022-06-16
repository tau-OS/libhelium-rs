// Borrowed from https://gitlab.gnome.org/World/Rust/libadwaita-rs/-/blob/master/libadwaita/src/application.rs

use crate::Application;

use glib::translate::*;
use std::cell::RefCell;
use std::rc::Rc;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;

impl Application {
    #[doc(alias = "he_application_new")]
    pub fn new(application_id: Option<&str>, flags: gio::ApplicationFlags) -> Self {
        skip_assert_initialized!();
        let app = unsafe {
            from_glib_full(ffi::he_application_new(
                application_id.to_glib_none().0,
                flags.into_glib(),
            ))
        };
        Self::register_startup_hook(&app);
        app
    }

    pub(crate) fn register_startup_hook(app: &Self) {
        skip_assert_initialized!();
        let signalid: Rc<RefCell<Option<SignalHandlerId>>> = Rc::new(RefCell::new(None));
        {
            let signalid_ = signalid.clone();

            let id = app.connect_startup(move |app| {
                app.disconnect(
                    signalid_
                        .borrow_mut()
                        .take()
                        .expect("Signal ID went missing"),
                );
                gtk::init().expect("Failed to initalize gtk4");
            });
            *signalid.borrow_mut() = Some(id);
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        let app = glib::object::Object::new::<Self>(&[])
            .expect("Can't construct Application object with default parameters");
        Self::register_startup_hook(&app);
        app
    }
}
