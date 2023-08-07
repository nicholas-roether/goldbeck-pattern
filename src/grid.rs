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

	let theme_ctx = use_context::<ThemeCtx>(cx).expect("TileOverlay is missing theme context!");

	let stroke_color = move || {
		let highlight = theme_ctx.highlight.get();
		if hovering() {
			highlight
		} else {
			String::from("none")
		}
	};

	view! { cx,
		<polygon
			class="cursor-crosshair"
			points=shape.svg_path()
			vector-effect="non-scaling-stroke"
			fill="transparent"
			stroke=stroke_color
			stroke-width="4"
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
				<clipPath id="centerSquare">
					<rect x="0" y="0" width=width height=height />
				</clipPath>
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
			<g clip-path="url(#centerSquare)">
				{move || tiling
					.with(|t| t.iter_lines())
					.map(|[p1, p2]| {
						view! { cx,
							<line
								x1=p1.x
								y1=p1.y
								x2=p2.x
								y2=p2.y
								stroke=theme_ctx.misc
								stroke-width="1"
								vector-effect="non-scaling-stroke"
							/>
						}
					})
					.collect_view(cx)
				}
			</g>
			{move || tiling
				.with(|t| t.iter_tiles())
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
fn Frame(cx: Scope, tiling: Memo<Tiling>) -> impl IntoView {
	let aspect_ratio = move || tiling.with(|t| t.viewport_width() / t.viewport_height());
	view! { cx,
		<div
			class="mw-100 mh-100 outline outline-2 outline-misc shadow-2xl"
			style:aspect-ratio=aspect_ratio
		/>
	}
}

#[component]
pub fn Grid(cx: Scope, format: ReadSignal<TilingFormat>) -> impl IntoView {
	let tiling = create_memo(cx, move |_| Tiling::load(format()));
	let colors = create_memo(cx, move |_| {
		GridColors::new(cx, tiling.with(|t| t.num_tiles()))
	});

	view! { cx,
		<div class="relative">
			<Pattern tiling colors />
			<div class="absolute inset-0 w-100 h-100 flex justify-center">
				<Frame tiling />
			</div>
			<div class="absolute inset-0 w-100 h-100">
				<Overlay tiling colors />
			</div>
		</div>
	}
}
