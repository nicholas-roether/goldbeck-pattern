#![feature(const_fn_floating_point_arithmetic)]

use std::{mem, panic};

use leptos::{ev::Event, leptos_dom::console_error, *};
use theme::{ThemeCtx, ThemeManager};

use crate::{export::export_svg, grid::Grid, theme::Theme, tiling::TilingFormat};

mod grid;

mod tiling;

mod export;

mod theme;

#[component]
fn App(cx: Scope) -> impl IntoView {
	let (format, set_format) = create_signal(cx, TilingFormat::Small);

	let theme_ctx = use_context::<ThemeCtx>(cx).expect("App is missing theme context!");

	let on_theme_change = move |evt: Event| {
		let value_str = event_target_value(&evt);
		let theme: u8 = value_str.parse().unwrap();
		unsafe { theme_ctx.set(mem::transmute(theme)) }
	};

	let on_format_change = move |evt: Event| {
		let value_str = event_target_value(&evt);
		let Ok(format): Result<u8, _> = value_str.parse() else {
			console_error(&format!(
				"Format dropdown had unexpected value ${value_str}"
			));
			return;
		};
		unsafe {
			set_format(mem::transmute(format));
		}
	};

	view! { cx,
		<label for="format">Format auswählen</label>
		<select id="format" on:change=on_format_change>
			<option value=TilingFormat::Small as u8>Klein</option>
			<option value=TilingFormat::Wide as u8>Breit</option>
			<option value=TilingFormat::Tall as u8>Hoch</option>
			<option value=TilingFormat::Large as u8>Groß</option>
		</select>
		<br />
		<label for="theme">Theme auswählen</label>
		<select id="theme" on:change=on_theme_change>
			<option value=Theme::Light as u8>Hell</option>
			<option value=Theme::Dark as u8>Dunkel</option>
		</select>
		<Grid format />
		<button on:click=move |_| export_svg()>Exportieren</button>
	}
}

fn main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|cx| view! { cx, <ThemeManager><App /></ThemeManager>})
}
