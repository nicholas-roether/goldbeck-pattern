use std::{mem, rc::Rc};

use leptos::{ev::Event, *};

use enum_iterator::all;

use crate::{
	components::pattern::Pattern,
	export::{export_pattern, OutputFormat},
	tiling::Tiling
};

use super::pattern::GridColors;

#[component]
pub fn ExportDialog(
	open: RwSignal<bool>,
	#[prop(into)] tiling: Signal<Rc<Tiling>>,
	#[prop(into)] colors: Signal<GridColors>
) -> impl IntoView {
	let (reps, set_reps) = create_signal(3);
	let (format, set_format) = create_signal(OutputFormat::Svg);

	let on_reps_change = move |ev: Event| {
		let value = event_target_value(&ev);
		set_reps(value.parse().expect("Range had unexpected value!"));
	};

	let on_format_change = move |ev: Event| {
		let value = event_target_value(&ev);
		let value_u8: u8 = value.parse().expect("Select had unexpected value!");
		set_format(unsafe { mem::transmute(value_u8) });
	};

	let on_export = move |_| {
		export_pattern("#export", "Pattern", format.get_untracked());
		open.set(false);
	};

	view! {
		<Show when=open>
			<div class="z-40 w-screen h-screen absolute inset-0 bg-background/70 flex items-center justify-center overflow-hidden">
				<section class="flex-1 max-w-2xl p-4 m-4 bg-primary text-primaryText shadow-xl">
					<div class="flex mb-4">
						<h1 class="flex-1 font-bold text-xl">"Muster exportieren"</h1>
						<button
							class="inline-block"
							aria-label="Schließen"
							on:click=move |_| open.set(false)
						>
							<box-icon name="x" color="currentColor"></box-icon>
						</button>
					</div>
					<div class="mb-4 p-4 flex justify-center h-80 bg-misc shadow-inner">
						<Pattern id="export" export=true tiling colors reps_x=reps reps_y=1/>
					</div>
					<div class="flex flex-col w-full sm:flex-row justify-between gap-4">
						<span class="inline-flex flex-col">
							<label for="exportRepsRange">Wiederholungen: {reps}</label>
							<br/>
							<input
								type="range"
								id="exportRepsRange"
								min="1"
								max="6"
								value=reps
								on:input=on_reps_change
							/>
						</span>
						<span class="flex-1"></span>
						<span class="inline-block h-12 bg-background text-backgroundText relative isolate">
							<select
								aria-label="Dateiformat"
								on:change=on_format_change
								class="appearance-none bg-transparent pl-6 pr-8 py-2 h-full"
							>
								{all::<OutputFormat>()
									.map(|opt_format| {
										view! {
											<option
												value=opt_format as u8
												selected=move || format() == opt_format
											>
												{opt_format.to_string()}
											</option>
										}
									})
									.collect_view()}
							</select>
							<box-icon
								class="absolute right-0 z-[-1] h-full mx-1"
								name="chevron-down"
								color="currentColor"
							></box-icon>
						</span>
						<button
							class="px-6 py-2 h-12 bg-secondary text-secondaryText hover:outline outline-2 outline-highlight"
							on:click=on_export
						>
							"Exportieren"
						</button>
					</div>
				</section>
			</div>
		</Show>
	}
}

