use std::iter;

use leptos::{ev::MouseEvent, *};

use crate::{
	theme::ThemeCtx,
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
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("Tile is missing theme context!");

	let fill_color = move || {
		let bg = theme_ctx.background.get();
		let active = theme_ctx.primary.get();
		if color() {
			active
		} else {
			bg
		}
	};
	view! { cx,
		<polygon
			points=shape.svg_path()
			fill=fill_color
			shape-rendering="crispEdges"
		/>
	}
}

const REPETITIONS_X: f32 = 3.0;
const REPETITIONS_Y: f32 = 1.0;

#[component]
fn Pattern(cx: Scope, tiling: Memo<Tiling>, colors: Memo<GridColors>) -> impl IntoView {
	let width = move || tiling.with(|t| (t.viewport_width() * REPETITIONS_X).to_string());
	let height = move || tiling.with(|t| (t.viewport_height() * REPETITIONS_Y).to_string());
	let view_box = move || format!("0 0 {} {}", width(), height());
	let pattern_width = move || tiling.with(|t| t.viewport_width().to_string());
	let pattern_height = move || tiling.with(|t| t.viewport_height().to_string());
	let pattern_view_box = move || format!("0 0 {} {}", pattern_width(), pattern_height());

	view! { cx,
		<svg id="pattern-svg" class="block" viewBox=view_box>
			<defs>
				<pattern
					id="Tiling"
					x="0"
					y="0"
					width=pattern_width
					height=pattern_height
					patternUnits="userSpaceOnUse"
					view_box=pattern_view_box
				>
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
				</pattern>
			</defs>
			<rect fill="url(#Tiling)" width=width height=height />
		</svg>
	}
}

#[component]
fn TileOverlay(cx: Scope, shape: Shape, color: RwSignal<bool>) -> impl IntoView {
	let (hovering, set_hovering) = create_signal(cx, false);

	let brush = move |buttons: u16| match buttons {
		0b01 => color.set(true),
		0b10 => color.set(false),
		_ => ()
	};

	let on_mouse_enter = move |evt: MouseEvent| {
		set_hovering(true);
		brush(evt.buttons());
	};

	let on_mouse_leave = move |_: MouseEvent| {
		set_hovering(false);
	};

	let on_mouse_down = move |evt: MouseEvent| {
		evt.prevent_default();
		brush(evt.buttons());
	};

	view! { cx,
		<polygon
			class="cursor-crosshair"
			points=shape.svg_path()
			vector-effect="non-scaling-stroke"
			fill="transparent"
			stroke=move || if hovering() { "red" } else { "none" }
			stroke-width="3"
			stroke-linejoin="round"
			on:mouseenter=on_mouse_enter
			on:mouseleave=on_mouse_leave
			on:mousedown=on_mouse_down
			on:contextmenu=|evt| evt.prevent_default()
		/>
	}
}

#[component]
fn Overlay(cx: Scope, tiling: Memo<Tiling>, colors: Memo<GridColors>) -> impl IntoView {
	let view_box = move || {
		tiling.with(|t| {
			format!(
				"{} {} {} {}",
				t.viewport_width() * (1.0 - REPETITIONS_X) / 2.0,
				t.viewport_height() * (1.0 - REPETITIONS_Y) / 2.0,
				t.viewport_width() * REPETITIONS_X,
				t.viewport_height() * REPETITIONS_Y
			)
		})
	};
	let width = move || tiling.with(|t| t.viewport_width());
	let height = move || tiling.with(|t| t.viewport_height());

	let theme_ctx = use_context::<ThemeCtx>(cx).expect("Overlay is missing theme context!");

	view! { cx,
		<svg viewBox=view_box class="block">
			<defs>
				<linearGradient id="fadeoutLeft">
					<stop offset="0%" stop-color=theme_ctx.background stop-opacity="1" />
					<stop offset="100%" stop-color=theme_ctx.background stop-opacity="0" />
				</linearGradient>
				<linearGradient id="fadeoutRight">
					<stop offset="0%" stop-color=theme_ctx.background stop-opacity="0" />
					<stop offset="100%" stop-color=theme_ctx.background stop-opacity="1" />
				</linearGradient>
			</defs>
			<rect
				x=move || -width()
				y="0"
				width=width
				height=height
				fill="url(#fadeoutLeft)"
			/>
			<rect
				x=width
				y="0"
				width=width
				height=height
				fill="url(#fadeoutRight)"
			/>
			<line
				x1="0"
				y1="0%"
				x2="0"
				y2="100%"
				stroke="black"
				stroke-width="2"
				vector-effect="non-scaling-stroke"
			/>
			<line
				x1=width
				y1="0%"
				x2=width
				y2="100%"
				stroke="black"
				stroke-width="2"
				vector-effect="non-scaling-stroke"
			/>
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

	view! { cx,
		<div class="relative overflow-hidden">
			<Pattern tiling colors />
			<div class="absolute inset-0 w-100 h-100">
				<Overlay tiling colors />
			</div>
		</div>
	}
}
