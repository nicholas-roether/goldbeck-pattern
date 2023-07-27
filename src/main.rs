#![feature(const_fn_floating_point_arithmetic)]

use std::panic;

use leptos::*;

use crate::grid::Grid;

mod css;

mod grid;

mod tiling;

#[component]
fn App(cx: Scope) -> impl IntoView {
	let container = css! {
		max-width: 500px;
	};

	let (scale, set_scale) = create_signal(cx, 1);

	view! { cx,
		<input
			type="range"
			min="1"
			max="3"
			prop:value={scale}
			on:input=move |ev| {
				if let Ok(value) = event_target_value(&ev).parse() {
					set_scale(value);
				}
			}
		/>
		<div class={container}>
			<Grid scale />
		</div>
	}
}

fn main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|cx| view! { cx, <App />})
}
