use std::rc::Rc;

use leptos::*;

use crate::{
	css,
	tiling::{Shape, Tiling, TilingFormat}
};

#[derive(Debug, Clone)]
struct TileState {
	shape: Rc<Shape>,
	active: RwSignal<bool>,
	outline: RwSignal<bool>
}

impl TileState {
	fn new(cx: Scope, shape: Shape) -> Self {
		Self {
			shape: Rc::new(shape),
			active: create_rw_signal(cx, false),
			outline: create_rw_signal(cx, false)
		}
	}
}

#[derive(Debug, Clone)]
struct GridState {
	gridlines: RwSignal<bool>,
	view_box: String,
	tiles: Rc<Vec<TileState>>
}

impl GridState {
	fn new(cx: Scope, tiling: Tiling) -> Self {
		let tiles = tiling
			.iter_tiles()
			.map(|shape| TileState::new(cx, shape))
			.collect();
		Self {
			gridlines: create_rw_signal(cx, true),
			view_box: tiling.view_box(),
			tiles: Rc::new(tiles)
		}
	}
}

#[component]
fn Tile(cx: Scope, state: TileState) -> impl IntoView {
	view! { cx,
		<polygon
			points=state.shape.svg_path()
			fill=move || if state.active.get() { "gray" } else { "white" }
			shape-rendering="crispEdges"
		/>
	}
}

#[component]
fn Pattern<FnGS>(cx: Scope, state: FnGS) -> impl IntoView
where
	FnGS: Fn() -> GridState + Copy + 'static
{
	view! { cx,
		<svg id="pattern-svg" viewBox=move || state().view_box>
			{move || state()
				.tiles
				.iter()
				.cloned()
				.map(|state| {
					view! { cx,
						<Tile state />
					}
				})
				.collect_view(cx)
			}
		</svg>
	}
}

#[component]
fn TileOverlay(cx: Scope, state: TileState) -> impl IntoView {
	view! { cx,
		<polygon
			points=state.shape.svg_path()
			fill="transparent"
			stroke=move || if state.outline.get() { "red" } else { "none" }
			stroke-width="0.025"
			on:click=move |_| state.active.set(!state.active.get())
			on:mouseenter=move |_| state.outline.set(true)
			on:mouseleave=move |_| state.outline.set(false)
		/>
	}
}

#[component]
fn Overlay<FnGS>(cx: Scope, state: FnGS) -> impl IntoView
where
	FnGS: Fn() -> GridState + Copy + 'static
{
	view! { cx,
		<svg viewBox=move || state().view_box>
			{move || state()
				.tiles
				.iter()
				.cloned()
				.map(|state| {
					view! { cx,
						<TileOverlay state />
					}
				})
				.collect_view(cx)
			}
		</svg>
	}
}

#[component]
pub fn Grid(cx: Scope, format: ReadSignal<TilingFormat>) -> impl IntoView {
	let tiling = move || Tiling::load(format());
	let state = move || GridState::new(cx, tiling());

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
			<Pattern state />
			<div class=overlay>
				<Overlay state />
			</div>
		</div>
	}
}
