use std::iter;

use leptos::*;

use crate::{
	theme::ThemeCtx,
	tiling::{Shape, Tiling}
};

#[derive(Debug, PartialEq)]
pub struct GridColors(Vec<RwSignal<bool>>);

impl GridColors {
	pub fn new(cx: Scope, size: usize) -> Self {
		Self(
			iter::repeat_with(|| create_rw_signal(cx, false))
				.take(size)
				.collect()
		)
	}

	pub fn get_color(&self, i: usize) -> RwSignal<bool> {
		self.0[i]
	}
}

#[component]
fn Tile(cx: Scope, shape: Shape, color: Signal<bool>) -> impl IntoView {
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

#[component]
pub fn Pattern(
	cx: Scope,
	tiling: Signal<Tiling>,
	colors: Signal<GridColors>,
	reps_x: usize,
	reps_y: usize,
	#[prop(optional)] faded: Option<bool>
) -> impl IntoView {
	let width = move || tiling.with(|t| (t.viewport_width() * reps_x as f32).to_string());
	let height = move || tiling.with(|t| (t.viewport_height() * reps_y as f32).to_string());
	let view_box = move || format!("0 0 {} {}", width(), height());
	let pattern_width = move || tiling.with(|t| t.viewport_width().to_string());
	let pattern_height = move || tiling.with(|t| t.viewport_height().to_string());
	let pattern_view_box = move || format!("0 0 {} {}", pattern_width(), pattern_height());

	let mut class = String::from("block");
	if faded.unwrap_or(false) {
		class += " opacity-75"
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
							view! { cx,
								<Tile shape color=colors.with(|c| c.get_color(i)).into() />
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
