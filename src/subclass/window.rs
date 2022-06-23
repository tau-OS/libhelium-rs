use crate::Window;
use gtk::subclass::prelude::*;
use gtk::subclass::window::WindowImpl as GtkWindowImpl;

pub trait WindowImpl: GtkWindowImpl {}

unsafe impl<T: WindowImpl> IsSubclassable<T> for Window {}
