use std::rc::Rc;

use leptos::{ev::Event, *};

use crate::{components::pattern::Pattern, export::export_svg, tiling::Tiling};

use super::pattern::GridColors;

#[component]
pub fn ExportDialog(
	cx: Scope,
	open: RwSignal<bool>,
	#[prop(into)] tiling: Signal<Rc<Tiling>>,
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
			<div class="z-40 w-screen h-screen absolute inset-0 bg-background/70 flex items-center justify-center overflow-hidden">
				<section class="flex-1 max-w-2xl p-4 m-4 bg-primary text-primaryText shadow-xl">
					<div class="flex mb-4">
						<h1 class="flex-1 font-bold text-xl">"Muster Exportieren"</h1>
						<button class="inline-block" aria-label="Schließen" on:click=move |_| open.set(false)>
							<box-icon name="x" color="currentColor" />
						</button>
					</div>
					<div class="mb-4 p-4 flex justify-center h-80 bg-misc shadow-inner">
						<Pattern id="export" export=true tiling colors reps_x=reps reps_y=1 />
					</div>
					<div class="flex flex-col w-full sm:flex-row justify-between gap-4">
						<span class="inline-flex flex-col">
							<label for="exportRepsRange">Wiederholungen: {reps}</label><br />
							<input type="range" id="exportRepsRange" min="1" max="6" value=reps on:input=on_reps_change />
						</span>
						<button
							class="px-6 py-2 h-12 bg-secondary text-secondaryText hover:outline outline-2 outline-highlight"
							on:click=on_export
						>"Exportieren"</button>
					</div>
				</section>
			</div>
		</Show>
	}
}
