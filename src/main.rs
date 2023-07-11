use leptos::*;

use crate::grid::Tile;

mod css;

mod grid;

#[component]
fn App(cx: Scope) -> impl IntoView {
	view! { cx,
		<Tile />
	}
}

fn main() {
	println!("Hello, world!");
	mount_to_body(|cx| view! { cx, <App />})
}
