#![feature(const_fn_floating_point_arithmetic)]

use std::panic;

use leptos::*;

use crate::{grid::Grid, tiling::TilingFormat};

mod css;

mod grid;

mod tiling;

#[component]
fn App(cx: Scope) -> impl IntoView {
	let container = css! {
		max-width: 500px;
	};

	let (format, set_format) = create_signal(cx, TilingFormat::Small);

	view! { cx,
		<div class={container}>
			<Grid format />
		</div>
	}
}

fn main() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));

	mount_to_body(|cx| view! { cx, <App />})
}
