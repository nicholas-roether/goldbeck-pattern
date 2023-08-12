use leptos::{ev::Event, *};

use crate::{components::pattern::Pattern, export::export_svg, tiling::Tiling};

use super::pattern::GridColors;

#[component]
pub fn ExportDialog(
	cx: Scope,
	open: RwSignal<bool>,
	#[prop(into)] tiling: Signal<Tiling>,
	#[prop(into)] colors: Signal<GridColors>
) -> impl IntoView {
	let (reps, set_reps) = create_signal(cx, 3);

	let on_reps_change = move |ev: Event| {
		let value = event_target_value(&ev);
		set_reps(value.parse().expect("Range had unexpected value!"));
	};

	let on_export = move |_| {
		export_svg("#export", "Pattern.svg");
		open.set(false);
	};

	view! { cx,
		<Show when=open fallback=|_| () >
			<div class="z-40 w-screen h-screen absolute inset-0 bg-background/70 flex items-center justify-center">
				<section class="w-full max-w-2xl p-4 m-4 bg-primary text-primaryText shadow-xl">
					<div class="flex pb-4">
						<h1 class="flex-1 font-bold text-xl">"Muster Exportieren"</h1>
						<button class="inline-block" aria-label="Schließen" on:click=move |_| open.set(false)>
							<box-icon name="x" color="currentColor" />
						</button>
					</div>
					<div class="flex justify-center h-80 bg-misc shadow-inner">
						<Pattern id="export" export=true tiling colors reps_x=reps reps_y=1 />
					</div>
					<input type="range" min="1" max="6" value=reps on:input=on_reps_change />
					<button on:click=on_export>"Exportieren"</button>
				</section>
			</div>
		</Show>
	}
}
