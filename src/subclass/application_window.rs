use crate::ApplicationWindow;
use gtk::subclass::prelude::*;
use gtk::subclass::application_window::ApplicationWindowImpl as GtkWindowImpl;

pub trait ApplicationWindowImpl: GtkWindowImpl {}

unsafe impl<T: ApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
