#![feature(const_fn_floating_point_arithmetic)]

use std::{mem, panic};

use leptos::{ev::Event, leptos_dom::console_error, *};

use crate::{grid::Grid, tiling::TilingFormat};

mod css;

mod grid;

mod tiling;

#[component]
fn App(cx: Scope) -> impl IntoView {
	let container = css! {
		border: 2px solid black;
	};

	let (format, set_format) = create_signal(cx, TilingFormat::Small);

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
		<div class={container}>
			<Grid format />
		</div>
	}
}

fn main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|cx| view! { cx, <App />})
}
