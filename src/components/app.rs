use leptos::*;

use crate::{
	components::{editor::Editor, pattern::GridColors, theme_selector::ThemeSelector},
	tiling::{Tiling, TilingFormat}
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
	let format = create_rw_signal(cx, TilingFormat::Small);
	let tiling = create_memo(cx, move |_| Tiling::load(format()));
	let colors = create_memo(cx, move |_| {
		GridColors::new(cx, tiling.with(|t| t.num_tiles()))
	});

	view! { cx,
		<ThemeSelector />
		<Editor tiling=tiling.into() colors=colors.into() />
	}
}
