// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Bin;
use crate::Button;
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
    #[doc(alias = "HeMiniContentBlock")]
    pub struct MiniContentBlock(Object<ffi::HeMiniContentBlock, ffi::HeMiniContentBlockClass>) @extends Bin, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_mini_content_block_get_type(),
    }
}

impl MiniContentBlock {
        pub const NONE: Option<&'static MiniContentBlock> = None;
    

    #[doc(alias = "he_mini_content_block_new_with_details")]
    pub fn with_details(title: Option<&str>, subtitle: Option<&str>, primary_button: Option<&impl IsA<Button>>) -> MiniContentBlock {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_new_with_details(title.to_glib_none().0, subtitle.to_glib_none().0, primary_button.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    #[doc(alias = "he_mini_content_block_new")]
    pub fn new() -> MiniContentBlock {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_new())
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`MiniContentBlock`] objects.
            ///
            /// This method returns an instance of [`MiniContentBlockBuilder`](crate::builders::MiniContentBlockBuilder) which can be used to create [`MiniContentBlock`] objects.
            pub fn builder() -> MiniContentBlockBuilder {
                MiniContentBlockBuilder::default()
            }
        
}

impl Default for MiniContentBlock {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`MiniContentBlock`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MiniContentBlockBuilder {
    title: Option<String>,
    subtitle: Option<String>,
    icon: Option<String>,
    gicon: Option<gio::Icon>,
    primary_button: Option<Button>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<gtk::Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<gtk::LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<gtk::Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<gtk::Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<gtk::AccessibleRole>,
}

impl MiniContentBlockBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`MiniContentBlockBuilder`].
    pub fn new() -> Self {
        Self::default()
    }


    // rustdoc-stripper-ignore-next
    /// Build the [`MiniContentBlock`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MiniContentBlock {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
if let Some(ref title) = self.title {
                properties.push(("title", title));
            }
if let Some(ref subtitle) = self.subtitle {
                properties.push(("subtitle", subtitle));
            }
if let Some(ref icon) = self.icon {
                properties.push(("icon", icon));
            }
if let Some(ref gicon) = self.gicon {
                properties.push(("gicon", gicon));
            }
if let Some(ref primary_button) = self.primary_button {
                properties.push(("primary-button", primary_button));
            }
if let Some(ref can_focus) = self.can_focus {
                properties.push(("can-focus", can_focus));
            }
if let Some(ref can_target) = self.can_target {
                properties.push(("can-target", can_target));
            }
if let Some(ref css_classes) = self.css_classes {
                properties.push(("css-classes", css_classes));
            }
if let Some(ref css_name) = self.css_name {
                properties.push(("css-name", css_name));
            }
if let Some(ref cursor) = self.cursor {
                properties.push(("cursor", cursor));
            }
if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
if let Some(ref focusable) = self.focusable {
                properties.push(("focusable", focusable));
            }
if let Some(ref halign) = self.halign {
                properties.push(("halign", halign));
            }
if let Some(ref has_tooltip) = self.has_tooltip {
                properties.push(("has-tooltip", has_tooltip));
            }
if let Some(ref height_request) = self.height_request {
                properties.push(("height-request", height_request));
            }
if let Some(ref hexpand) = self.hexpand {
                properties.push(("hexpand", hexpand));
            }
if let Some(ref hexpand_set) = self.hexpand_set {
                properties.push(("hexpand-set", hexpand_set));
            }
if let Some(ref layout_manager) = self.layout_manager {
                properties.push(("layout-manager", layout_manager));
            }
if let Some(ref margin_bottom) = self.margin_bottom {
                properties.push(("margin-bottom", margin_bottom));
            }
if let Some(ref margin_end) = self.margin_end {
                properties.push(("margin-end", margin_end));
            }
if let Some(ref margin_start) = self.margin_start {
                properties.push(("margin-start", margin_start));
            }
if let Some(ref margin_top) = self.margin_top {
                properties.push(("margin-top", margin_top));
            }
if let Some(ref name) = self.name {
                properties.push(("name", name));
            }
if let Some(ref opacity) = self.opacity {
                properties.push(("opacity", opacity));
            }
if let Some(ref overflow) = self.overflow {
                properties.push(("overflow", overflow));
            }
if let Some(ref receives_default) = self.receives_default {
                properties.push(("receives-default", receives_default));
            }
if let Some(ref sensitive) = self.sensitive {
                properties.push(("sensitive", sensitive));
            }
if let Some(ref tooltip_markup) = self.tooltip_markup {
                properties.push(("tooltip-markup", tooltip_markup));
            }
if let Some(ref tooltip_text) = self.tooltip_text {
                properties.push(("tooltip-text", tooltip_text));
            }
if let Some(ref valign) = self.valign {
                properties.push(("valign", valign));
            }
if let Some(ref vexpand) = self.vexpand {
                properties.push(("vexpand", vexpand));
            }
if let Some(ref vexpand_set) = self.vexpand_set {
                properties.push(("vexpand-set", vexpand_set));
            }
if let Some(ref visible) = self.visible {
                properties.push(("visible", visible));
            }
if let Some(ref width_request) = self.width_request {
                properties.push(("width-request", width_request));
            }
if let Some(ref accessible_role) = self.accessible_role {
                properties.push(("accessible-role", accessible_role));
            }
        glib::Object::new::<MiniContentBlock>(&properties)
                .expect("Failed to create an instance of MiniContentBlock")

    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn gicon(mut self, gicon: &impl IsA<gio::Icon>) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn primary_button(mut self, primary_button: &impl IsA<Button>) -> Self {
        self.primary_button = Some(primary_button.clone().upcast());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: gtk::Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: gtk::AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

pub trait MiniContentBlockExt: 'static {
    #[doc(alias = "he_mini_content_block_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "he_mini_content_block_set_title")]
    fn set_title(&self, value: &str);

    #[doc(alias = "he_mini_content_block_get_subtitle")]
    #[doc(alias = "get_subtitle")]
    fn subtitle(&self) -> Option<glib::GString>;

    #[doc(alias = "he_mini_content_block_set_subtitle")]
    fn set_subtitle(&self, value: &str);

    #[doc(alias = "he_mini_content_block_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<glib::GString>;

    #[doc(alias = "he_mini_content_block_set_icon")]
    fn set_icon(&self, value: &str);

    #[doc(alias = "he_mini_content_block_set_gicon")]
    fn set_gicon(&self, value: &impl IsA<gio::Icon>);

    #[doc(alias = "he_mini_content_block_get_primary_button")]
    #[doc(alias = "get_primary_button")]
    fn primary_button(&self) -> Option<Button>;

    #[doc(alias = "he_mini_content_block_set_primary_button")]
    fn set_primary_button(&self, value: &impl IsA<Button>);

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "subtitle")]
    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "gicon")]
    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "primary-button")]
    fn connect_primary_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MiniContentBlock>> MiniContentBlockExt for O {
    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_get_title(self.as_ref().to_glib_none().0))
        }
    }

    fn set_title(&self, value: &str) {
        unsafe {
            ffi::he_mini_content_block_set_title(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn subtitle(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_get_subtitle(self.as_ref().to_glib_none().0))
        }
    }

    fn set_subtitle(&self, value: &str) {
        unsafe {
            ffi::he_mini_content_block_set_subtitle(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn set_icon(&self, value: &str) {
        unsafe {
            ffi::he_mini_content_block_set_icon(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_gicon(&self, value: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::he_mini_content_block_set_gicon(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0);
        }
    }

    fn primary_button(&self) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::he_mini_content_block_get_primary_button(self.as_ref().to_glib_none().0))
        }
    }

    fn set_primary_button(&self, value: &impl IsA<Button>) {
        unsafe {
            ffi::he_mini_content_block_set_primary_button(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0);
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<MiniContentBlock>, F: Fn(&P) + 'static>(this: *mut ffi::HeMiniContentBlock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MiniContentBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_title_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P: IsA<MiniContentBlock>, F: Fn(&P) + 'static>(this: *mut ffi::HeMiniContentBlock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MiniContentBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subtitle_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P: IsA<MiniContentBlock>, F: Fn(&P) + 'static>(this: *mut ffi::HeMiniContentBlock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MiniContentBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P: IsA<MiniContentBlock>, F: Fn(&P) + 'static>(this: *mut ffi::HeMiniContentBlock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MiniContentBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gicon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_primary_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_primary_button_trampoline<P: IsA<MiniContentBlock>, F: Fn(&P) + 'static>(this: *mut ffi::HeMiniContentBlock, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MiniContentBlock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::primary-button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_primary_button_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MiniContentBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MiniContentBlock")
    }
}
