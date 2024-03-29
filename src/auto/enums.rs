// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeAboutWindowLicenses")]
pub enum AboutWindowLicenses {
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_GPLv3")]
    Gplv3,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_MIT")]
    Mit,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_MPLv2")]
    Mplv2,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_UNLICENSE")]
    Unlicense,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_APACHEv2")]
    Apachev2,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_WTFPL")]
    Wtfpl,
    #[doc(alias = "HE_ABOUT_WINDOW_LICENSES_PROPRIETARY")]
    Proprietary,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AboutWindowLicenses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AboutWindowLicenses::{}",
            match *self {
                Self::Gplv3 => "Gplv3",
                Self::Mit => "Mit",
                Self::Mplv2 => "Mplv2",
                Self::Unlicense => "Unlicense",
                Self::Apachev2 => "Apachev2",
                Self::Wtfpl => "Wtfpl",
                Self::Proprietary => "Proprietary",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for AboutWindowLicenses {
    type GlibType = ffi::HeAboutWindowLicenses;

    fn into_glib(self) -> ffi::HeAboutWindowLicenses {
        match self {
            Self::Gplv3 => ffi::HE_ABOUT_WINDOW_LICENSES_GPLv3,
            Self::Mit => ffi::HE_ABOUT_WINDOW_LICENSES_MIT,
            Self::Mplv2 => ffi::HE_ABOUT_WINDOW_LICENSES_MPLv2,
            Self::Unlicense => ffi::HE_ABOUT_WINDOW_LICENSES_UNLICENSE,
            Self::Apachev2 => ffi::HE_ABOUT_WINDOW_LICENSES_APACHEv2,
            Self::Wtfpl => ffi::HE_ABOUT_WINDOW_LICENSES_WTFPL,
            Self::Proprietary => ffi::HE_ABOUT_WINDOW_LICENSES_PROPRIETARY,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeAboutWindowLicenses> for AboutWindowLicenses {
    unsafe fn from_glib(value: ffi::HeAboutWindowLicenses) -> Self {
        match value {
            ffi::HE_ABOUT_WINDOW_LICENSES_GPLv3 => Self::Gplv3,
            ffi::HE_ABOUT_WINDOW_LICENSES_MIT => Self::Mit,
            ffi::HE_ABOUT_WINDOW_LICENSES_MPLv2 => Self::Mplv2,
            ffi::HE_ABOUT_WINDOW_LICENSES_UNLICENSE => Self::Unlicense,
            ffi::HE_ABOUT_WINDOW_LICENSES_APACHEv2 => Self::Apachev2,
            ffi::HE_ABOUT_WINDOW_LICENSES_WTFPL => Self::Wtfpl,
            ffi::HE_ABOUT_WINDOW_LICENSES_PROPRIETARY => Self::Proprietary,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AboutWindowLicenses {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_about_window_licenses_get_type()) }
    }
}

impl glib::value::ValueType for AboutWindowLicenses {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AboutWindowLicenses {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AboutWindowLicenses {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeBannerStyle")]
pub enum BannerStyle {
    #[doc(alias = "HE_BANNER_STYLE_INFO")]
    Info,
    #[doc(alias = "HE_BANNER_STYLE_WARNING")]
    Warning,
    #[doc(alias = "HE_BANNER_STYLE_ERROR")]
    Error,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BannerStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BannerStyle::{}",
            match *self {
                Self::Info => "Info",
                Self::Warning => "Warning",
                Self::Error => "Error",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for BannerStyle {
    type GlibType = ffi::HeBannerStyle;

    fn into_glib(self) -> ffi::HeBannerStyle {
        match self {
            Self::Info => ffi::HE_BANNER_STYLE_INFO,
            Self::Warning => ffi::HE_BANNER_STYLE_WARNING,
            Self::Error => ffi::HE_BANNER_STYLE_ERROR,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeBannerStyle> for BannerStyle {
    unsafe fn from_glib(value: ffi::HeBannerStyle) -> Self {
        match value {
            ffi::HE_BANNER_STYLE_INFO => Self::Info,
            ffi::HE_BANNER_STYLE_WARNING => Self::Warning,
            ffi::HE_BANNER_STYLE_ERROR => Self::Error,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for BannerStyle {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_banner_style_get_type()) }
    }
}

impl glib::value::ValueType for BannerStyle {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BannerStyle {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BannerStyle {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeBottomBarPosition")]
pub enum BottomBarPosition {
    #[doc(alias = "HE_BOTTOM_BAR_POSITION_LEFT")]
    Left,
    #[doc(alias = "HE_BOTTOM_BAR_POSITION_RIGHT")]
    Right,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BottomBarPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BottomBarPosition::{}",
            match *self {
                Self::Left => "Left",
                Self::Right => "Right",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for BottomBarPosition {
    type GlibType = ffi::HeBottomBarPosition;

    fn into_glib(self) -> ffi::HeBottomBarPosition {
        match self {
            Self::Left => ffi::HE_BOTTOM_BAR_POSITION_LEFT,
            Self::Right => ffi::HE_BOTTOM_BAR_POSITION_RIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeBottomBarPosition> for BottomBarPosition {
    unsafe fn from_glib(value: ffi::HeBottomBarPosition) -> Self {
        match value {
            ffi::HE_BOTTOM_BAR_POSITION_LEFT => Self::Left,
            ffi::HE_BOTTOM_BAR_POSITION_RIGHT => Self::Right,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for BottomBarPosition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_bottom_bar_position_get_type()) }
    }
}

impl glib::value::ValueType for BottomBarPosition {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BottomBarPosition {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BottomBarPosition {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeColors")]
pub enum Colors {
    #[doc(alias = "HE_COLORS_NONE")]
    None,
    #[doc(alias = "HE_COLORS_RED")]
    Red,
    #[doc(alias = "HE_COLORS_ORANGE")]
    Orange,
    #[doc(alias = "HE_COLORS_YELLOW")]
    Yellow,
    #[doc(alias = "HE_COLORS_GREEN")]
    Green,
    #[doc(alias = "HE_COLORS_BLUE")]
    Blue,
    #[doc(alias = "HE_COLORS_INDIGO")]
    Indigo,
    #[doc(alias = "HE_COLORS_PURPLE")]
    Purple,
    #[doc(alias = "HE_COLORS_PINK")]
    Pink,
    #[doc(alias = "HE_COLORS_MINT")]
    Mint,
    #[doc(alias = "HE_COLORS_BROWN")]
    Brown,
    #[doc(alias = "HE_COLORS_LIGHT")]
    Light,
    #[doc(alias = "HE_COLORS_DARK")]
    Dark,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Colors::{}",
            match *self {
                Self::None => "None",
                Self::Red => "Red",
                Self::Orange => "Orange",
                Self::Yellow => "Yellow",
                Self::Green => "Green",
                Self::Blue => "Blue",
                Self::Indigo => "Indigo",
                Self::Purple => "Purple",
                Self::Pink => "Pink",
                Self::Mint => "Mint",
                Self::Brown => "Brown",
                Self::Light => "Light",
                Self::Dark => "Dark",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Colors {
    type GlibType = ffi::HeColors;

    fn into_glib(self) -> ffi::HeColors {
        match self {
            Self::None => ffi::HE_COLORS_NONE,
            Self::Red => ffi::HE_COLORS_RED,
            Self::Orange => ffi::HE_COLORS_ORANGE,
            Self::Yellow => ffi::HE_COLORS_YELLOW,
            Self::Green => ffi::HE_COLORS_GREEN,
            Self::Blue => ffi::HE_COLORS_BLUE,
            Self::Indigo => ffi::HE_COLORS_INDIGO,
            Self::Purple => ffi::HE_COLORS_PURPLE,
            Self::Pink => ffi::HE_COLORS_PINK,
            Self::Mint => ffi::HE_COLORS_MINT,
            Self::Brown => ffi::HE_COLORS_BROWN,
            Self::Light => ffi::HE_COLORS_LIGHT,
            Self::Dark => ffi::HE_COLORS_DARK,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeColors> for Colors {
    unsafe fn from_glib(value: ffi::HeColors) -> Self {
        match value {
            ffi::HE_COLORS_NONE => Self::None,
            ffi::HE_COLORS_RED => Self::Red,
            ffi::HE_COLORS_ORANGE => Self::Orange,
            ffi::HE_COLORS_YELLOW => Self::Yellow,
            ffi::HE_COLORS_GREEN => Self::Green,
            ffi::HE_COLORS_BLUE => Self::Blue,
            ffi::HE_COLORS_INDIGO => Self::Indigo,
            ffi::HE_COLORS_PURPLE => Self::Purple,
            ffi::HE_COLORS_PINK => Self::Pink,
            ffi::HE_COLORS_MINT => Self::Mint,
            ffi::HE_COLORS_BROWN => Self::Brown,
            ffi::HE_COLORS_LIGHT => Self::Light,
            ffi::HE_COLORS_DARK => Self::Dark,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Colors {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_colors_get_type()) }
    }
}

impl glib::value::ValueType for Colors {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Colors {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Colors {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeContentBlockImageClusterImagePosition")]
pub enum ContentBlockImageClusterImagePosition {
    #[doc(alias = "HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_LEFT")]
    TopLeft,
    #[doc(alias = "HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_LEFT")]
    BottomLeft,
    #[doc(alias = "HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_RIGHT")]
    TopRight,
    #[doc(alias = "HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_RIGHT")]
    BottomRight,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ContentBlockImageClusterImagePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ContentBlockImageClusterImagePosition::{}",
            match *self {
                Self::TopLeft => "TopLeft",
                Self::BottomLeft => "BottomLeft",
                Self::TopRight => "TopRight",
                Self::BottomRight => "BottomRight",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ContentBlockImageClusterImagePosition {
    type GlibType = ffi::HeContentBlockImageClusterImagePosition;

    fn into_glib(self) -> ffi::HeContentBlockImageClusterImagePosition {
        match self {
            Self::TopLeft => ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_LEFT,
            Self::BottomLeft => ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_LEFT,
            Self::TopRight => ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_RIGHT,
            Self::BottomRight => ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_RIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeContentBlockImageClusterImagePosition>
    for ContentBlockImageClusterImagePosition
{
    unsafe fn from_glib(value: ffi::HeContentBlockImageClusterImagePosition) -> Self {
        match value {
            ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_LEFT => Self::TopLeft,
            ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_LEFT => Self::BottomLeft,
            ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_TOP_RIGHT => Self::TopRight,
            ffi::HE_CONTENT_BLOCK_IMAGE_CLUSTER_IMAGE_POSITION_BOTTOM_RIGHT => Self::BottomRight,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ContentBlockImageClusterImagePosition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_content_block_image_cluster_image_position_get_type()) }
    }
}

impl glib::value::ValueType for ContentBlockImageClusterImagePosition {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ContentBlockImageClusterImagePosition {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ContentBlockImageClusterImagePosition {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeDesktopColorScheme")]
pub enum DesktopColorScheme {
    #[doc(alias = "HE_DESKTOP_COLOR_SCHEME_NO_PREFERENCE")]
    NoPreference,
    #[doc(alias = "HE_DESKTOP_COLOR_SCHEME_DARK")]
    Dark,
    #[doc(alias = "HE_DESKTOP_COLOR_SCHEME_LIGHT")]
    Light,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DesktopColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DesktopColorScheme::{}",
            match *self {
                Self::NoPreference => "NoPreference",
                Self::Dark => "Dark",
                Self::Light => "Light",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DesktopColorScheme {
    type GlibType = ffi::HeDesktopColorScheme;

    fn into_glib(self) -> ffi::HeDesktopColorScheme {
        match self {
            Self::NoPreference => ffi::HE_DESKTOP_COLOR_SCHEME_NO_PREFERENCE,
            Self::Dark => ffi::HE_DESKTOP_COLOR_SCHEME_DARK,
            Self::Light => ffi::HE_DESKTOP_COLOR_SCHEME_LIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeDesktopColorScheme> for DesktopColorScheme {
    unsafe fn from_glib(value: ffi::HeDesktopColorScheme) -> Self {
        match value {
            ffi::HE_DESKTOP_COLOR_SCHEME_NO_PREFERENCE => Self::NoPreference,
            ffi::HE_DESKTOP_COLOR_SCHEME_DARK => Self::Dark,
            ffi::HE_DESKTOP_COLOR_SCHEME_LIGHT => Self::Light,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for DesktopColorScheme {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_desktop_color_scheme_get_type()) }
    }
}

impl glib::value::ValueType for DesktopColorScheme {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DesktopColorScheme {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for DesktopColorScheme {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeModifierBadgeAlignment")]
pub enum ModifierBadgeAlignment {
    #[doc(alias = "HE_MODIFIER_BADGE_ALIGNMENT_LEFT")]
    Left,
    #[doc(alias = "HE_MODIFIER_BADGE_ALIGNMENT_CENTER")]
    Center,
    #[doc(alias = "HE_MODIFIER_BADGE_ALIGNMENT_RIGHT")]
    Right,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ModifierBadgeAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ModifierBadgeAlignment::{}",
            match *self {
                Self::Left => "Left",
                Self::Center => "Center",
                Self::Right => "Right",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ModifierBadgeAlignment {
    type GlibType = ffi::HeModifierBadgeAlignment;

    fn into_glib(self) -> ffi::HeModifierBadgeAlignment {
        match self {
            Self::Left => ffi::HE_MODIFIER_BADGE_ALIGNMENT_LEFT,
            Self::Center => ffi::HE_MODIFIER_BADGE_ALIGNMENT_CENTER,
            Self::Right => ffi::HE_MODIFIER_BADGE_ALIGNMENT_RIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeModifierBadgeAlignment> for ModifierBadgeAlignment {
    unsafe fn from_glib(value: ffi::HeModifierBadgeAlignment) -> Self {
        match value {
            ffi::HE_MODIFIER_BADGE_ALIGNMENT_LEFT => Self::Left,
            ffi::HE_MODIFIER_BADGE_ALIGNMENT_CENTER => Self::Center,
            ffi::HE_MODIFIER_BADGE_ALIGNMENT_RIGHT => Self::Right,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for ModifierBadgeAlignment {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_modifier_badge_alignment_get_type()) }
    }
}

impl glib::value::ValueType for ModifierBadgeAlignment {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ModifierBadgeAlignment {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ModifierBadgeAlignment {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeOverlayButtonAlignment")]
pub enum OverlayButtonAlignment {
    #[doc(alias = "HE_OVERLAY_BUTTON_ALIGNMENT_LEFT")]
    Left,
    #[doc(alias = "HE_OVERLAY_BUTTON_ALIGNMENT_CENTER")]
    Center,
    #[doc(alias = "HE_OVERLAY_BUTTON_ALIGNMENT_RIGHT")]
    Right,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for OverlayButtonAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OverlayButtonAlignment::{}",
            match *self {
                Self::Left => "Left",
                Self::Center => "Center",
                Self::Right => "Right",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for OverlayButtonAlignment {
    type GlibType = ffi::HeOverlayButtonAlignment;

    fn into_glib(self) -> ffi::HeOverlayButtonAlignment {
        match self {
            Self::Left => ffi::HE_OVERLAY_BUTTON_ALIGNMENT_LEFT,
            Self::Center => ffi::HE_OVERLAY_BUTTON_ALIGNMENT_CENTER,
            Self::Right => ffi::HE_OVERLAY_BUTTON_ALIGNMENT_RIGHT,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeOverlayButtonAlignment> for OverlayButtonAlignment {
    unsafe fn from_glib(value: ffi::HeOverlayButtonAlignment) -> Self {
        match value {
            ffi::HE_OVERLAY_BUTTON_ALIGNMENT_LEFT => Self::Left,
            ffi::HE_OVERLAY_BUTTON_ALIGNMENT_CENTER => Self::Center,
            ffi::HE_OVERLAY_BUTTON_ALIGNMENT_RIGHT => Self::Right,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for OverlayButtonAlignment {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_overlay_button_alignment_get_type()) }
    }
}

impl glib::value::ValueType for OverlayButtonAlignment {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for OverlayButtonAlignment {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for OverlayButtonAlignment {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeOverlayButtonSize")]
pub enum OverlayButtonSize {
    #[doc(alias = "HE_OVERLAY_BUTTON_SIZE_SMALL")]
    Small,
    #[doc(alias = "HE_OVERLAY_BUTTON_SIZE_MEDIUM")]
    Medium,
    #[doc(alias = "HE_OVERLAY_BUTTON_SIZE_LARGE")]
    Large,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for OverlayButtonSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OverlayButtonSize::{}",
            match *self {
                Self::Small => "Small",
                Self::Medium => "Medium",
                Self::Large => "Large",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for OverlayButtonSize {
    type GlibType = ffi::HeOverlayButtonSize;

    fn into_glib(self) -> ffi::HeOverlayButtonSize {
        match self {
            Self::Small => ffi::HE_OVERLAY_BUTTON_SIZE_SMALL,
            Self::Medium => ffi::HE_OVERLAY_BUTTON_SIZE_MEDIUM,
            Self::Large => ffi::HE_OVERLAY_BUTTON_SIZE_LARGE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeOverlayButtonSize> for OverlayButtonSize {
    unsafe fn from_glib(value: ffi::HeOverlayButtonSize) -> Self {
        match value {
            ffi::HE_OVERLAY_BUTTON_SIZE_SMALL => Self::Small,
            ffi::HE_OVERLAY_BUTTON_SIZE_MEDIUM => Self::Medium,
            ffi::HE_OVERLAY_BUTTON_SIZE_LARGE => Self::Large,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for OverlayButtonSize {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_overlay_button_size_get_type()) }
    }
}

impl glib::value::ValueType for OverlayButtonSize {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for OverlayButtonSize {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for OverlayButtonSize {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "HeTabSwitcherTabBarBehavior")]
pub enum TabSwitcherTabBarBehavior {
    #[doc(alias = "HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_ALWAYS")]
    Always,
    #[doc(alias = "HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_SINGLE")]
    Single,
    #[doc(alias = "HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_NEVER")]
    Never,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TabSwitcherTabBarBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TabSwitcherTabBarBehavior::{}",
            match *self {
                Self::Always => "Always",
                Self::Single => "Single",
                Self::Never => "Never",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for TabSwitcherTabBarBehavior {
    type GlibType = ffi::HeTabSwitcherTabBarBehavior;

    fn into_glib(self) -> ffi::HeTabSwitcherTabBarBehavior {
        match self {
            Self::Always => ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_ALWAYS,
            Self::Single => ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_SINGLE,
            Self::Never => ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_NEVER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::HeTabSwitcherTabBarBehavior> for TabSwitcherTabBarBehavior {
    unsafe fn from_glib(value: ffi::HeTabSwitcherTabBarBehavior) -> Self {
        match value {
            ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_ALWAYS => Self::Always,
            ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_SINGLE => Self::Single,
            ffi::HE_TAB_SWITCHER_TAB_BAR_BEHAVIOR_NEVER => Self::Never,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for TabSwitcherTabBarBehavior {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::he_tab_switcher_tab_bar_behavior_get_type()) }
    }
}

impl glib::value::ValueType for TabSwitcherTabBarBehavior {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TabSwitcherTabBarBehavior {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for TabSwitcherTabBarBehavior {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
