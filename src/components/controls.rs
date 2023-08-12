use leptos::*;

use crate::components::pattern::TileColor;

#[component]
fn BrushButton(
	cx: Scope,
	name: &'static str,
	icon: &'static str,
	color: TileColor,
	brush: RwSignal<TileColor>
) -> impl IntoView {
	view! { cx,
		<button
			role="radio"
			aria-label=name
			class=move || String::new()
				+ "inline-flex p-2 "
				+ match color {
					TileColor::Primary => "bg-primary text-primaryText ",
					TileColor::Secondary => "bg-secondary text-secondaryText ",
					TileColor::None => "bg-background text-backgroundText "
				}
				+ if brush() == color { "relative z-100 outline outline-3 outline-highlight " } else { " " }
			on:click=move |_| brush.set(color)
		>
			<box-icon name=icon size="md" color="currentColor" />
		</button>
	}
}

#[component]
fn BrushControls(cx: Scope, brush: RwSignal<TileColor>) -> impl IntoView {
	view! { cx,
		<div role="radiogroup" aria-label="Werkzeug auswählen" class="flex w-full max-w-sm border-2 border-misc">
			<BrushButton name="Pinsel 1" icon="brush" color=TileColor::Primary brush />
			<BrushButton name="Pinsel 2" icon="brush" color=TileColor::Secondary brush />
			<BrushButton name="Radiergummi" icon="eraser" color=TileColor::None brush />
		</div>
	}
}

#[component]
pub fn Controls(cx: Scope, brush: RwSignal<TileColor>, exporting: RwSignal<bool>) -> impl IntoView {
	view! { cx,
		<div class="p-6">
			<BrushControls brush />
			<button on:click=move|_| exporting.set(true)>"Export"</button>
		</div>
	}
}
