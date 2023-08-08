use std::iter;

use leptos::*;

use crate::{
	theme::ThemeCtx,
	tiling::{Shape, Tiling}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
	None,
	Primary,
	Secondary
}

impl Default for TileColor {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Debug, PartialEq)]
pub struct GridColors(Vec<RwSignal<TileColor>>);

impl GridColors {
	pub fn new(cx: Scope, size: usize) -> Self {
		Self(
			iter::repeat_with(|| create_rw_signal(cx, TileColor::default()))
				.take(size)
				.collect()
		)
	}

	pub fn get_color(&self, i: usize) -> RwSignal<TileColor> {
		self.0[i]
	}
}

fn get_fill_color(cx: Scope, color: Signal<TileColor>) -> impl Fn() -> String {
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("Tile is missing theme context!");
	move || {
		let bg = theme_ctx.with_value(|tc| tc.background.get());
		let primary = theme_ctx.with_value(|tc| tc.primary.get());
		let secondary = theme_ctx.with_value(|tc| tc.secondary.get());
		match color() {
			TileColor::None => bg,
			TileColor::Primary => primary,
			TileColor::Secondary => secondary
		}
	}
}

#[component]
fn ExportTile(cx: Scope, shape: Shape, color: Signal<TileColor>) -> impl IntoView {
	view! { cx,
		<polygon
			points=shape.svg_path()
			fill=move || get_fill_color(cx, color)
			shape-rendering="crispEdges"
		/>
	}
}

#[component]
fn Tile(cx: Scope, shape: Shape, color: Signal<TileColor>) -> impl IntoView {
	let class = move || match color() {
		TileColor::Primary => "fill-primary",
		TileColor::Secondary => "fill-secondary",
		TileColor::None => "fill-transparent"
	};

	view! { cx,
		<polygon
			class=class
			points=shape.svg_path()
			shape-rendering="crispEdges"
		/>
	}
}

#[component]
pub fn Pattern(
	cx: Scope,
	#[prop(into)] tiling: Signal<Tiling>,
	#[prop(into)] colors: Signal<GridColors>,
	reps_x: usize,
	reps_y: usize,
	#[prop(default = false)] background: bool,
	#[prop(default = false)] export: bool
) -> impl IntoView {
	let width = move || tiling.with(|t| (t.viewport_width() * reps_x as f32).to_string());
	let height = move || tiling.with(|t| (t.viewport_height() * reps_y as f32).to_string());
	let view_box = move || format!("0 0 {} {}", width(), height());
	let pattern_width = move || tiling.with(|t| t.viewport_width().to_string());
	let pattern_height = move || tiling.with(|t| t.viewport_height().to_string());
	let pattern_view_box = move || format!("0 0 {} {}", pattern_width(), pattern_height());

	let mut class = String::from("block");
	if background {
		class += " opacity-75 -z-1"
	};

	view! { cx,
		<svg id="pattern-svg" class=class viewBox=view_box>
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
							let color = colors.with(|c| c.get_color(i)).into();
							if export {
								view! { cx, <ExportTile shape color /> }
							} else {
								view! { cx, <Tile shape color /> }
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
