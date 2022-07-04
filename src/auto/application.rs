// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColorRGBColor;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "HeApplication")]
    pub struct Application(Object<ffi::HeApplication, ffi::HeApplicationClass>) @extends gtk::Application, gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        type_ => || ffi::he_application_get_type(),
    }
}

impl Application {
        pub const NONE: Option<&'static Application> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Application`] objects.
            ///
            /// This method returns an instance of [`ApplicationBuilder`](crate::builders::ApplicationBuilder) which can be used to create [`Application`] objects.
            pub fn builder() -> ApplicationBuilder {
                ApplicationBuilder::default()
            }
        
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Application`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ApplicationBuilder {
    default_accent_color: Option<ColorRGBColor>,
    accent_color: Option<String>,
    foreground: Option<String>,
    accent_foreground: Option<String>,
    menubar: Option<gio::MenuModel>,
    register_session: Option<bool>,
    action_group: Option<gio::ActionGroup>,
    application_id: Option<String>,
    flags: Option<gio::ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ApplicationBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`Application`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref default_accent_color) = self.default_accent_color {
                properties.push(("default-accent-color", default_accent_color));
            }
if let Some(ref accent_color) = self.accent_color {
                properties.push(("accent-color", accent_color));
            }
if let Some(ref foreground) = self.foreground {
                properties.push(("foreground", foreground));
            }
if let Some(ref accent_foreground) = self.accent_foreground {
                properties.push(("accent-foreground", accent_foreground));
            }
if let Some(ref menubar) = self.menubar {
                properties.push(("menubar", menubar));
            }
if let Some(ref register_session) = self.register_session {
                properties.push(("register-session", register_session));
            }
if let Some(ref action_group) = self.action_group {
                properties.push(("action-group", action_group));
            }
if let Some(ref application_id) = self.application_id {
                properties.push(("application-id", application_id));
            }
if let Some(ref flags) = self.flags {
                properties.push(("flags", flags));
            }
if let Some(ref inactivity_timeout) = self.inactivity_timeout {
                properties.push(("inactivity-timeout", inactivity_timeout));
            }
if let Some(ref resource_base_path) = self.resource_base_path {
                properties.push(("resource-base-path", resource_base_path));
            }
        let ret = glib::Object::new::<Application>(&properties)
                .expect("Failed to create an instance of Application");
        {
            Application::register_startup_hook(&ret);
        }
    ret
    }

    pub fn default_accent_color(mut self, default_accent_color: &ColorRGBColor) -> Self {
        self.default_accent_color = Some(default_accent_color.clone());
        self
    }

    pub fn accent_color(mut self, accent_color: &str) -> Self {
        self.accent_color = Some(accent_color.to_string());
        self
    }

    pub fn foreground(mut self, foreground: &str) -> Self {
        self.foreground = Some(foreground.to_string());
        self
    }

    pub fn accent_foreground(mut self, accent_foreground: &str) -> Self {
        self.accent_foreground = Some(accent_foreground.to_string());
        self
    }

    pub fn menubar(mut self, menubar: &impl IsA<gio::MenuModel>) -> Self {
        self.menubar = Some(menubar.clone().upcast());
        self
    }

    pub fn register_session(mut self, register_session: bool) -> Self {
        self.register_session = Some(register_session);
        self
    }

    pub fn action_group(mut self, action_group: &impl IsA<gio::ActionGroup>) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: gio::ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

pub trait HeApplicationExt: 'static {
    #[doc(alias = "he_application_get_default_accent_color")]
    #[doc(alias = "get_default_accent_color")]
    fn default_accent_color(&self) -> Option<ColorRGBColor>;

    #[doc(alias = "he_application_get_accent_color")]
    #[doc(alias = "get_accent_color")]
    fn accent_color(&self) -> Option<glib::GString>;

    #[doc(alias = "he_application_get_foreground")]
    #[doc(alias = "get_foreground")]
    fn foreground(&self) -> Option<glib::GString>;

    #[doc(alias = "he_application_get_accent_foreground")]
    #[doc(alias = "get_accent_foreground")]
    fn accent_foreground(&self) -> Option<glib::GString>;

    #[doc(alias = "accent-color")]
    fn set_accent_color(&self, accent_color: Option<&str>);

    fn set_foreground(&self, foreground: Option<&str>);

    #[doc(alias = "accent-foreground")]
    fn set_accent_foreground(&self, accent_foreground: Option<&str>);

    #[doc(alias = "default-accent-color")]
    fn connect_default_accent_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "accent-color")]
    fn connect_accent_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "foreground")]
    fn connect_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "accent-foreground")]
    fn connect_accent_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application>> HeApplicationExt for O {
    fn default_accent_color(&self) -> Option<ColorRGBColor> {
        unsafe {
            from_glib_none(ffi::he_application_get_default_accent_color(self.as_ref().to_glib_none().0))
        }
    }

    fn accent_color(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_application_get_accent_color(self.as_ref().to_glib_none().0))
        }
    }

    fn foreground(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_application_get_foreground(self.as_ref().to_glib_none().0))
        }
    }

    fn accent_foreground(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_application_get_accent_foreground(self.as_ref().to_glib_none().0))
        }
    }

    fn set_accent_color(&self, accent_color: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(),"accent-color", &accent_color)
    }

    fn set_foreground(&self, foreground: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(),"foreground", &foreground)
    }

    fn set_accent_foreground(&self, accent_foreground: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(),"accent-foreground", &accent_foreground)
    }

    fn connect_default_accent_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_accent_color_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::HeApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-accent-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_default_accent_color_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_accent_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accent_color_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::HeApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accent-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_accent_color_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_foreground_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::HeApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::foreground\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_foreground_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_accent_foreground_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accent_foreground_trampoline<P: IsA<Application>, F: Fn(&P) + 'static>(this: *mut ffi::HeApplication, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accent-foreground\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_accent_foreground_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Application")
    }
}
