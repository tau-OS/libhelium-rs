// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Button;
use crate::Colors;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    ///
    ///
    /// # Implements
    ///
    /// [`ButtonExt`][trait@crate::prelude::ButtonExt], [`trait@gtk::prelude::ButtonExt`], [`trait@gtk::prelude::WidgetExt`], [`trait@glib::ObjectExt`], [`trait@gtk::prelude::AccessibleExt`], [`trait@gtk::prelude::BuildableExt`], [`trait@gtk::prelude::ConstraintTargetExt`], [`trait@gtk::prelude::ActionableExt`]
    // rustdoc-stripper-ignore-next-stop
    ///
    ///
    /// # Implements
    ///
    /// [`ButtonExt`][trait@crate::prelude::ButtonExt], [`trait@gtk::prelude::ButtonExt`], [`trait@gtk::prelude::WidgetExt`], [`trait@glib::ObjectExt`], [`trait@gtk::prelude::AccessibleExt`], [`trait@gtk::prelude::BuildableExt`], [`trait@gtk::prelude::ConstraintTargetExt`], [`trait@gtk::prelude::ActionableExt`]
    #[doc(alias = "HeFillButton")]
    pub struct FillButton(Object<ffi::HeFillButton, ffi::HeFillButtonClass>) @extends Button, gtk::Button, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;

    match fn {
        type_ => || ffi::he_fill_button_get_type(),
    }
}

impl FillButton {
    pub const NONE: Option<&'static FillButton> = None;

    #[doc(alias = "he_fill_button_new")]
    pub fn new(label: &str) -> FillButton {
        unsafe { from_glib_none(ffi::he_fill_button_new(label.to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FillButton`] objects.
    ///
    /// This method returns an instance of [`FillButtonBuilder`](crate::builders::FillButtonBuilder) which can be used to create [`FillButton`] objects.
    pub fn builder() -> FillButtonBuilder {
        FillButtonBuilder::default()
    }
}

impl Default for FillButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
            .expect("Can't construct FillButton object with default parameters")
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FillButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FillButtonBuilder {
    color: Option<Colors>,
    icon: Option<String>,
    child: Option<gtk::Widget>,
    has_frame: Option<bool>,
    icon_name: Option<String>,
    label: Option<String>,
    use_underline: Option<bool>,
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
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
}

impl FillButtonBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FillButtonBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FillButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FillButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref color) = self.color {
            properties.push(("color", color));
        }
        if let Some(ref icon) = self.icon {
            properties.push(("icon", icon));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref has_frame) = self.has_frame {
            properties.push(("has-frame", has_frame));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
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
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        glib::Object::new::<FillButton>(&properties)
            .expect("Failed to create an instance of FillButton")
    }

    pub fn color(mut self, color: Colors) -> Self {
        self.color = Some(color);
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// The child widget.
    // rustdoc-stripper-ignore-next-stop
    /// The child widget.
    // rustdoc-stripper-ignore-next-stop
    /// The child widget.
    pub fn child(mut self, child: &impl IsA<gtk::Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    /// Whether the button has a frame.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the button has a frame.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the button has a frame.
    pub fn has_frame(mut self, has_frame: bool) -> Self {
        self.has_frame = Some(has_frame);
        self
    }

    /// The name of the icon used to automatically populate the button.
    // rustdoc-stripper-ignore-next-stop
    /// The name of the icon used to automatically populate the button.
    // rustdoc-stripper-ignore-next-stop
    /// The name of the icon used to automatically populate the button.
    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    /// Text of the label inside the button, if the button contains a label widget.
    // rustdoc-stripper-ignore-next-stop
    /// Text of the label inside the button, if the button contains a label widget.
    // rustdoc-stripper-ignore-next-stop
    /// Text of the label inside the button, if the button contains a label widget.
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// If set, an underline in the text indicates that the following character is
    /// to be used as mnemonic.
    // rustdoc-stripper-ignore-next-stop
    /// If set, an underline in the text indicates that the following character is
    /// to be used as mnemonic.
    // rustdoc-stripper-ignore-next-stop
    /// If set, an underline in the text indicates that the following character is
    /// to be used as mnemonic.
    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    /// Whether the widget or any of its descendents can accept
    /// the input focus.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget or any of its descendents can accept
    /// the input focus.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget or any of its descendents can accept
    /// the input focus.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    /// Whether the widget can receive pointer events.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget can receive pointer events.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget can receive pointer events.
    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    /// A list of css classes applied to this widget.
    // rustdoc-stripper-ignore-next-stop
    /// A list of css classes applied to this widget.
    // rustdoc-stripper-ignore-next-stop
    /// A list of css classes applied to this widget.
    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    /// The name of this widget in the CSS tree.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// The name of this widget in the CSS tree.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// The name of this widget in the CSS tree.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    /// The cursor used by @widget.
    // rustdoc-stripper-ignore-next-stop
    /// The cursor used by @widget.
    // rustdoc-stripper-ignore-next-stop
    /// The cursor used by @widget.
    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    /// Whether the widget should grab focus when it is clicked with the mouse.
    ///
    /// This property is only relevant for widgets that can take focus.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget should grab focus when it is clicked with the mouse.
    ///
    /// This property is only relevant for widgets that can take focus.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget should grab focus when it is clicked with the mouse.
    ///
    /// This property is only relevant for widgets that can take focus.
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    /// Whether this widget itself will accept the input focus.
    // rustdoc-stripper-ignore-next-stop
    /// Whether this widget itself will accept the input focus.
    // rustdoc-stripper-ignore-next-stop
    /// Whether this widget itself will accept the input focus.
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    /// How to distribute horizontal space if widget gets extra space.
    // rustdoc-stripper-ignore-next-stop
    /// How to distribute horizontal space if widget gets extra space.
    // rustdoc-stripper-ignore-next-stop
    /// How to distribute horizontal space if widget gets extra space.
    pub fn halign(mut self, halign: gtk::Align) -> Self {
        self.halign = Some(halign);
        self
    }

    /// Enables or disables the emission of the ::query-tooltip signal on @widget.
    ///
    /// A value of [`true`] indicates that @widget can have a tooltip, in this case
    /// the widget will be queried using `signal::gtk::Widget::query-tooltip` to
    /// determine whether it will provide a tooltip or not.
    // rustdoc-stripper-ignore-next-stop
    /// Enables or disables the emission of the ::query-tooltip signal on @widget.
    ///
    /// A value of [`true`] indicates that @widget can have a tooltip, in this case
    /// the widget will be queried using `signal::gtk::Widget::query-tooltip` to
    /// determine whether it will provide a tooltip or not.
    // rustdoc-stripper-ignore-next-stop
    /// Enables or disables the emission of the ::query-tooltip signal on @widget.
    ///
    /// A value of [`true`] indicates that @widget can have a tooltip, in this case
    /// the widget will be queried using `signal::gtk::Widget::query-tooltip` to
    /// determine whether it will provide a tooltip or not.
    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    /// Override for height request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    // rustdoc-stripper-ignore-next-stop
    /// Override for height request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    // rustdoc-stripper-ignore-next-stop
    /// Override for height request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    /// Whether to expand horizontally.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to expand horizontally.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to expand horizontally.
    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    /// Whether to use the `hexpand` property.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to use the `hexpand` property.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to use the `hexpand` property.
    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    /// The [`gtk::LayoutManager`][crate::gtk::LayoutManager] instance to use to compute the preferred size
    /// of the widget, and allocate its children.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// The [`gtk::LayoutManager`][crate::gtk::LayoutManager] instance to use to compute the preferred size
    /// of the widget, and allocate its children.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// The [`gtk::LayoutManager`][crate::gtk::LayoutManager] instance to use to compute the preferred size
    /// of the widget, and allocate its children.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn layout_manager(mut self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    /// Margin on bottom side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on bottom side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on bottom side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    /// Margin on end of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on end of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on end of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    /// Margin on start of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on start of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on start of widget, horizontally.
    ///
    /// This property supports left-to-right and right-to-left text
    /// directions.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    /// Margin on top side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on top side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    // rustdoc-stripper-ignore-next-stop
    /// Margin on top side of widget.
    ///
    /// This property adds margin outside of the widget's normal size
    /// request, the margin will be added in addition to the size from
    /// [`WidgetExtManual::set_size_request()`][crate::gtk::prelude::WidgetExtManual::set_size_request()] for example.
    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    /// The name of the widget.
    // rustdoc-stripper-ignore-next-stop
    /// The name of the widget.
    // rustdoc-stripper-ignore-next-stop
    /// The name of the widget.
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// The requested opacity of the widget.
    // rustdoc-stripper-ignore-next-stop
    /// The requested opacity of the widget.
    // rustdoc-stripper-ignore-next-stop
    /// The requested opacity of the widget.
    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// How content outside the widget's content area is treated.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// How content outside the widget's content area is treated.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    // rustdoc-stripper-ignore-next-stop
    /// How content outside the widget's content area is treated.
    ///
    /// This property is meant to be set by widget implementations,
    /// typically in their instance init function.
    pub fn overflow(mut self, overflow: gtk::Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    /// Whether the widget will receive the default action when it is focused.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget will receive the default action when it is focused.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget will receive the default action when it is focused.
    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    /// Whether the widget responds to input.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget responds to input.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget responds to input.
    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    /// Sets the text of tooltip to be the given string, which is marked up
    /// with Pango markup.
    ///
    /// Also see `Gtk::Tooltip::set_markup()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    // rustdoc-stripper-ignore-next-stop
    /// Sets the text of tooltip to be the given string, which is marked up
    /// with Pango markup.
    ///
    /// Also see `Gtk::Tooltip::set_markup()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    // rustdoc-stripper-ignore-next-stop
    /// Sets the text of tooltip to be the given string, which is marked up
    /// with Pango markup.
    ///
    /// Also see `Gtk::Tooltip::set_markup()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    /// Sets the text of tooltip to be the given string.
    ///
    /// Also see `Gtk::Tooltip::set_text()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    // rustdoc-stripper-ignore-next-stop
    /// Sets the text of tooltip to be the given string.
    ///
    /// Also see `Gtk::Tooltip::set_text()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    // rustdoc-stripper-ignore-next-stop
    /// Sets the text of tooltip to be the given string.
    ///
    /// Also see `Gtk::Tooltip::set_text()`.
    ///
    /// This is a convenience property which will take care of getting the
    /// tooltip shown if the given string is not [`None`]:
    /// `property::gtk::Widget::has-tooltip` will automatically be set to [`true`]
    /// and there will be taken care of `signal::gtk::Widget::query-tooltip` in
    /// the default signal handler.
    ///
    /// Note that if both `property::gtk::Widget::tooltip-text` and
    /// `property::gtk::Widget::tooltip-markup` are set, the last one wins.
    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    /// How to distribute vertical space if widget gets extra space.
    // rustdoc-stripper-ignore-next-stop
    /// How to distribute vertical space if widget gets extra space.
    // rustdoc-stripper-ignore-next-stop
    /// How to distribute vertical space if widget gets extra space.
    pub fn valign(mut self, valign: gtk::Align) -> Self {
        self.valign = Some(valign);
        self
    }

    /// Whether to expand vertically.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to expand vertically.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to expand vertically.
    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    /// Whether to use the `vexpand` property.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to use the `vexpand` property.
    // rustdoc-stripper-ignore-next-stop
    /// Whether to use the `vexpand` property.
    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    /// Whether the widget is visible.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget is visible.
    // rustdoc-stripper-ignore-next-stop
    /// Whether the widget is visible.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// Override for width request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    // rustdoc-stripper-ignore-next-stop
    /// Override for width request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    // rustdoc-stripper-ignore-next-stop
    /// Override for width request of the widget.
    ///
    /// If this is -1, the natural request will be used.
    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    /// The accessible role of the given [`gtk::Accessible`][crate::gtk::Accessible] implementation.
    ///
    /// The accessible role cannot be changed once set.
    // rustdoc-stripper-ignore-next-stop
    /// The accessible role of the given [`gtk::Accessible`][crate::gtk::Accessible] implementation.
    ///
    /// The accessible role cannot be changed once set.
    // rustdoc-stripper-ignore-next-stop
    /// The accessible role of the given [`gtk::Accessible`][crate::gtk::Accessible] implementation.
    ///
    /// The accessible role cannot be changed once set.
    pub fn accessible_role(mut self, accessible_role: gtk::AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }
}

impl fmt::Display for FillButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FillButton")
    }
}
