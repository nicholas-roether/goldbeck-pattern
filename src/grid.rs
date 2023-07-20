use leptos::*;

use crate::{
	css,
	tiling::{Shape, Tiling}
};

#[component]
fn Tile(cx: Scope, shape: Shape) -> impl IntoView {
	let (active, set_active) = create_signal(cx, false);

	let tile = css! {
		&:hover {
			fill: red;
		}
	};

	view! { cx,
		<polygon
			points=shape.svg_path()
			fill=move || if active() { "gray" } else { "white" }
			class={tile}
			on:click=move |_| set_active(!active())
		/>
	}
}

#[component]
pub fn Grid(cx: Scope, size: ReadSignal<usize>) -> impl IntoView {
	let tiling = move || Tiling::new(size());

	view! { cx,
		<svg viewBox="0 0 1 1" xmlns="http://www.w3.org/2000/svg" shape-rendering="crispEdges">
			<For
				each=move || tiling().tiles
				key=|(id, _)| *id
				view=move |cx, (_, shape)| {
					view! { cx,
						<Tile shape />
					}
				}
			/>
		</svg>
	}
}
