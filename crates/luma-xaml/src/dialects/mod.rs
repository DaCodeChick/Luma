//! XAML dialect support (WinUI 3, WPF, Generic).

#[cfg(feature = "winui3")]
pub mod winui3;

#[cfg(feature = "wpf")]
pub mod wpf;

#[cfg(feature = "generic")]
pub mod generic;
