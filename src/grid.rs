use leptos::*;

use crate::css;

#[component]
pub fn Tile(cx: Scope, x: usize, y: usize) -> impl IntoView {
	let tile = css! {
		flex: 1;
	};

	view! { cx,
		<div class={tile}>
			"x = "
			{x}
			<br />
			"y = "
			{y}
		</div>
	}
}

#[component]
pub fn Grid(cx: Scope, size: ReadSignal<usize>) -> impl IntoView {
	let grid = css! {
		width: 100%;
		aspect-ratio: 1;
		display: flex;
		flex-direction: column;
	};

	let gridLine = css! {
		height: 100%;
		display: flex;
		flex: 1;
	};

	view! { cx,
		<div class={grid}>
			<For
				each=move || (0..size.get())
				key=|x| *x
				view=move |cx, x| view! { cx,
					<div class={gridLine.clone()}>
						<For
							each=move || (0..size.get())
							key=|y| *y
							view=move |cx, y| view! { cx,
								<Tile x={x} y={y} />
							}
						/>
					</div>
				}
			/>
		</div>
	}
}
