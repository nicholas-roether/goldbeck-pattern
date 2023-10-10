use leptos::*;

#[derive(Clone, Copy)]
struct DialogContext(RwSignal<Option<RwSignal<bool>>>);

impl DialogContext {
	fn open_dialog(&self, open: RwSignal<bool>) {
		if let Some(prev_open) = self.0.get() {
			prev_open.set(false);
		}
		self.0.set(Some(open));
	}
}

#[component]
pub fn Dialog(
	cx: Scope,
	#[prop(into)] id: String,
	open: RwSignal<bool>,
	#[prop(into)] title: String,
	children: Children
) -> impl IntoView {
	let dialog_ctx = use_context::<DialogContext>(cx).expect("Missing DialogProvider!");
	let title_id = id.clone() + "__title";
	let description_id = id + "__description";

	create_effect(cx, move |_| {
		if open() {
			dialog_ctx.open_dialog(open);
		}
	});

	let children = store_value(cx, children);

	view! { cx,
		<Show when=open fallback=|_| ()>
			<div class="z-40 w-screen h-screen absolute inset-0 bg-background/70 flex items-center justify-center overflow-hidden">
				<section
					role="dialog"
					aria-labelledby=title_id.clone()
					aria-describedby=description_id.clone()
					class="fkex-1 max-w-2xl p-4 m-4 bg-primary text-primaryText shadow-xl"
				>
					<div class="flex mb-4">
						<h1
							id=title_id.clone()
							class="flex-1 font-bold text-xl"
						>
							{title.clone()}
						</h1>
						<button class="inline-block" aria-label="Schließen">
							<box-icon name="x" color="currentColor" />
						</button>
					</div>
					<div class="mb-4 p-4">
						{children.with_value(|children| children(cx))}
					</div>
				</section>
			</div>
		</Show>
	}
}
