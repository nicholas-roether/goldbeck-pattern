use std::iter;

use leptos::*;

use crate::{
	css,
	tiling::{Shape, Tiling, TilingFormat}
};

#[derive(Debug, PartialEq)]
struct GridColors(Vec<RwSignal<bool>>);

impl GridColors {
	fn new(cx: Scope, size: usize) -> Self {
		Self(
			iter::repeat_with(|| create_rw_signal(cx, false))
				.take(size)
				.collect()
		)
	}

	fn get_color(&self, i: usize) -> RwSignal<bool> {
		self.0[i]
	}
}

#[component]
fn Tile(cx: Scope, shape: Shape, color: RwSignal<bool>) -> impl IntoView {
	view! { cx,
		<polygon
			points=shape.svg_path()
			fill=move || if color() { "gray" } else { "white" }
			shape-rendering="crispEdges"
		/>
	}
}

#[component]
fn Pattern(cx: Scope, tiling: Memo<Tiling>, colors: Memo<GridColors>) -> impl IntoView {
	view! { cx,
		<svg id="pattern-svg" viewBox=move || tiling().view_box()>
			{move || tiling()
				.iter_tiles()
				.enumerate()
				.map(|(i, shape)| {
					view! { cx,
						<Tile shape color=colors.with(|c| c.get_color(i))/>
					}
				})
				.collect_view(cx)
			}
		</svg>
	}
}

#[component]
fn TileOverlay(cx: Scope, shape: Shape, color: RwSignal<bool>) -> impl IntoView {
	let (hovering, set_hovering) = create_signal(cx, false);
	view! { cx,
		<polygon
			points=shape.svg_path()
			fill="transparent"
			stroke=move || if hovering() { "red" } else { "none" }
			stroke-width="0.025"
			on:mouseenter=move |_| set_hovering(true)
			on:mouseleave=move |_| set_hovering(false)
			on:click=move |_| color.update(|c| *c = !*c)
		/>
	}
}

#[component]
fn Overlay(cx: Scope, tiling: Memo<Tiling>, colors: Memo<GridColors>) -> impl IntoView {
	view! { cx,
		<svg viewBox=move || tiling().view_box()>
			{move || tiling()
				.iter_tiles()
				.enumerate()
				.map(|(i, shape)| {
					view! { cx,
						<TileOverlay shape color=colors.with(|c| c.get_color(i)) />
					}
				})
				.collect_view(cx)
			}
		</svg>
	}
}

#[component]
pub fn Grid(cx: Scope, format: ReadSignal<TilingFormat>) -> impl IntoView {
	let tiling = create_memo(cx, move |_| Tiling::load(format()));
	let colors = create_memo(cx, move |_| {
		GridColors::new(cx, tiling.with(|t| t.num_tiles()))
	});

	let container = css! {
		position: relative;
	};
	let overlay = css! {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	};

	view! { cx,
		<div class=container>
			<Pattern tiling colors />
			<div class=overlay>
				<Overlay tiling colors />
			</div>
		</div>
	}
}
