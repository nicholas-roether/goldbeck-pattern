use std::mem;

use leptos::{ev::Event, *};

use crate::{cls, components::pattern::TileColor, tiling::TilingFormat};

#[component]
fn BrushButton(
	name: &'static str,
	icon: &'static str,
	color: TileColor,
	brush: RwSignal<TileColor>
) -> impl IntoView {
	view! {
		<button
			role="radio"
			aria-label=name
			class=move || cls! {
				"inline-flex h-full flex-1 justify-center items-center transition-colors sm:aspect-square",
				match color {
					TileColor::Primary => "bg-primary text-primaryText ",
					TileColor::Secondary => "bg-secondary text-secondaryText ",
					TileColor::None => "bg-background text-backgroundText "
				},
				if brush() == color { "relative z-100 outline outline-3 outline-highlight " } else { " " }
			}
			on:click=move |_| brush.set(color)
		>
			<box-icon name=icon size="md" color="currentColor" />
		</button>
	}
}

#[component]
fn BrushControls(brush: RwSignal<TileColor>) -> impl IntoView {
	view! {
		<div role="radiogroup" aria-label="Werkzeug auswählen" class="flex border-2 border-misc h-12">
			<BrushButton name="Pinsel 1" icon="brush" color=TileColor::Primary brush />
			<BrushButton name="Pinsel 2" icon="brush" color=TileColor::Secondary brush />
			<BrushButton name="Radiergummi" icon="eraser" color=TileColor::None brush />
		</div>
	}
}

#[component]
fn ExportButton(exporting: RwSignal<bool>) -> impl IntoView {
	view! {
		<button
			class="inline-flex justify-center items-center h-12 sm:aspect-square font-semibold border-2 border-misc transition-all hover:shadow-lg"
			on:click=move|_| exporting.set(true)
		>
			<box-icon name="download" size="md" color="currentColor" />
		</button>
	}
}

#[component]
fn FormatSelector(format: RwSignal<TilingFormat>) -> impl IntoView {
	let on_format_change = move |ev: Event| {
		let value = event_target_value(&ev);
		let value_u8: u8 = value.parse().unwrap();
		unsafe {
			format.set(mem::transmute(value_u8));
		}
	};

	view! {
		<select
			class="h-12 p-2 bg-transparent border-2 border-misc"
			aria-label="Format auswählen"
			on:change=on_format_change
		>
			<option value=TilingFormat::F5X5 as u8>"5×5"</option>
			<option value=TilingFormat::F10X10 as u8>"10×10"</option>
			<option value=TilingFormat::F10X15 as u8>"10×15"</option>
			<option value=TilingFormat::F15X15 as u8>"15×15"</option>
		</select>
	}
}

#[component]
pub fn Controls(
	brush: RwSignal<TileColor>,
	exporting: RwSignal<bool>,
	format: RwSignal<TilingFormat>
) -> impl IntoView {
	view! {
		<div class="p-3 sm:p-6 w-full max-w-sm flex justify-between gap-2 sm:gap-4 flex-col sm:flex-row">
			<FormatSelector format />
			<BrushControls brush />
			<ExportButton exporting />
		</div>
	}
}
