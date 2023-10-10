use std::rc::Rc;

use leptos::*;

use crate::{
	components::{
		canvas::Canvas,
		controls::Controls,
		export_dialog::ExportDialog,
		pattern::{GridColors, TileColor},
		theme_selector::ThemeSelector
	},
	tiling::{Tiling, TilingFormat}
};

#[component]
pub fn App() -> impl IntoView {
	let format = create_rw_signal(TilingFormat::F5X5);
	let tiling = create_memo(move |_| Rc::new(Tiling::load(format())));
	let colors = create_memo(move |_| GridColors::new(tiling.with(|t| t.num_tiles())));
	let brush = create_rw_signal(TileColor::Primary);
	let exporting = create_rw_signal(false);

	view! {
		<main class="w-screen h-screen flex flex-col items-center overflow-hidden">
			<ThemeSelector />
			<div class="w-full min-h-0 p-3 sm:px-16">
				<Canvas tiling colors brush />
			</div>
			<Controls brush exporting format />
			<ExportDialog open=exporting tiling colors />
		</main>
	}
}
