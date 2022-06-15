// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "HeView")]
    pub struct View(Object<ffi::HeView, ffi::HeViewClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_view_get_type(),
    }
}

impl View {
    pub const NONE: Option<&'static View> = None;
}

pub trait ViewExt: 'static {
    #[doc(alias = "he_view_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "he_view_set_title")]
    fn set_title(&self, value: &str);

    #[doc(alias = "he_view_get_stack")]
    #[doc(alias = "get_stack")]
    fn stack(&self) -> Option<gtk::Stack>;

    #[doc(alias = "he_view_set_stack")]
    fn set_stack(&self, value: &gtk::Stack);

    #[doc(alias = "he_view_get_subtitle")]
    #[doc(alias = "get_subtitle")]
    fn subtitle(&self) -> Option<glib::GString>;

    #[doc(alias = "he_view_set_subtitle")]
    fn set_subtitle(&self, value: &str);

    #[doc(alias = "he_view_add_child")]
    fn add_child(
        &self,
        builder: &gtk::Builder,
        child: &impl IsA<glib::Object>,
        type_: Option<&str>,
    );

    #[doc(alias = "he_view_add")]
    fn add(&self, widget: &impl IsA<gtk::Widget>);

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "stack")]
    fn connect_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "subtitle")]
    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<View>> ViewExt for O {
    fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::he_view_get_title(self.as_ref().to_glib_none().0)) }
    }

    fn set_title(&self, value: &str) {
        unsafe {
            ffi::he_view_set_title(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn stack(&self) -> Option<gtk::Stack> {
        unsafe { from_glib_none(ffi::he_view_get_stack(self.as_ref().to_glib_none().0)) }
    }

    fn set_stack(&self, value: &gtk::Stack) {
        unsafe {
            ffi::he_view_set_stack(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn subtitle(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::he_view_get_subtitle(self.as_ref().to_glib_none().0)) }
    }

    fn set_subtitle(&self, value: &str) {
        unsafe {
            ffi::he_view_set_subtitle(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn add_child(
        &self,
        builder: &gtk::Builder,
        child: &impl IsA<glib::Object>,
        type_: Option<&str>,
    ) {
        unsafe {
            ffi::he_view_add_child(
                self.as_ref().to_glib_none().0,
                builder.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
            );
        }
    }

    fn add(&self, widget: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_view_add(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stack_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stack\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stack_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for View {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("View")
    }
}
