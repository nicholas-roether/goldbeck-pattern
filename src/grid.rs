use leptos::*;

use crate::css;

#[component]
pub fn Tile(cx: Scope) -> impl IntoView {
	let tile = css! {
		width: 1fr;
		height: 1fr;
	};

	view! { cx,
		<div class={tile}>"Test"</div>
	}
}
