use leptos::*;

use crate::grid::Grid;

mod css;

mod grid;

#[component]
fn App(cx: Scope) -> impl IntoView {
	let (size, set_size) = create_signal(cx, 8);

	view! { cx,
		<input
			type="range"
			min="4"
			max="16"
			prop:value={size}
			on:input=move |ev| {
				if let Ok(value) = event_target_value(&ev).parse() {
					set_size(value);
				}
			}
		/>
		<Grid size />
	}
}

fn main() {
	println!("Hello, world!");
	mount_to_body(|cx| view! { cx, <App />})
}
