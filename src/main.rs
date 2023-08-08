#![feature(const_fn_floating_point_arithmetic)]

use std::panic;

use components::app::App;
use leptos::*;
use theme::ThemeManager;

mod components;

mod tiling;

mod export;

mod theme;

fn main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|cx| view! { cx, <ThemeManager><App /></ThemeManager>})
}
