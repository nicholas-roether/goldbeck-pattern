use std::{iter, rc::Rc};

use leptos::*;

use crate::{
	cls,
	theme::ThemeData,
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

#[derive(Debug, Clone, PartialEq)]
pub struct GridColors(Vec<RwSignal<TileColor>>);

impl GridColors {
	pub fn new(size: usize) -> Self {
		Self(
			iter::repeat_with(|| create_rw_signal(TileColor::default()))
				.take(size)
				.collect()
		)
	}

	pub fn get_color(&self, i: usize) -> RwSignal<TileColor> {
		self.0[i]
	}
}

#[component]
#[allow(clippy::needless_lifetimes)]
fn ExportTile<'a>(shape: Shape, color: TileColor, theme_data: &'a ThemeData) -> impl IntoView {
	let fill = match color {
		TileColor::Primary => theme_data.primary.clone(),
		TileColor::Secondary => theme_data.secondary.clone(),
		TileColor::None => theme_data.background.clone()
	};
	view! {
		<polygon points=shape.svg_path() fill=fill.clone() stroke=fill stroke-width="0.05"></polygon>
	}
}

#[component]
fn Tile(shape: Shape, #[prop(into)] color: Signal<TileColor>) -> impl IntoView {
	view! {
		<polygon
			class=move || {
				cls! {
					match color() { TileColor::Primary => "fill-primary stroke-primary",
					TileColor::Secondary => "fill-secondary stroke-secondary", TileColor::None =>
					"fill-transparent stroke-transparent" }
				}
			}

			points=shape.svg_path()
			stroke-width="2"
			vector-effect="non-scaling-stroke"
		></polygon>
	}
}

#[component]
fn ExportGrid(tiling: Rc<Tiling>, colors: GridColors) -> impl IntoView {
	let theme_data = ThemeData::load();
	move || {
		tiling
			.iter_tiles()
			.enumerate()
			.map(|(i, shape)| {
				let color = colors.get_color(i).get_untracked();
				view! { <ExportTile shape color theme_data=&theme_data/> }
			})
			.collect_view()
	}
}

#[component]
fn Grid(tiling: Signal<Rc<Tiling>>, colors: Signal<GridColors>) -> impl IntoView {
	move || {
		tiling
			.with(|t| t.iter_tiles())
			.enumerate()
			.map(|(i, shape)| {
				let color = colors.with(|c| c.get_color(i));
				view! { <Tile shape color/> }
			})
			.collect_view()
	}
}

#[component]
pub fn Pattern(
	id: &'static str,
	#[prop(into)] tiling: Signal<Rc<Tiling>>,
	#[prop(into)] colors: Signal<GridColors>,
	#[prop(into)] reps_x: MaybeSignal<usize>,
	#[prop(into)] reps_y: MaybeSignal<usize>,
	#[prop(default = false)] background: bool,
	#[prop(default = false)] export: bool
) -> impl IntoView {
	let width = move || tiling.with(|t| (t.viewport_width() * reps_x() as f32).to_string());
	let height = move || tiling.with(|t| (t.viewport_height() * reps_y() as f32).to_string());
	let view_box = move || format!("0 0 {} {}", width(), height());
	let pattern_width = move || tiling.with(|t| t.viewport_width().to_string());
	let pattern_height = move || tiling.with(|t| t.viewport_height().to_string());
	let pattern_view_box = move || format!("0 0 {} {}", pattern_width(), pattern_height());

	let mut class = String::from("block transition-none");
	if background {
		class += " opacity-75 -z-1"
	};

	let grid = if export {
		view! { <ExportGrid tiling=tiling.get_untracked() colors=colors.get_untracked()/> }
	} else {
		view! { <Grid tiling colors/> }
	};

	let tilingId = format!("Pattern-{id}__Tiling");

	view! {
		<svg class=class viewBox=view_box id=id>
			<defs>
				<pattern
					id=tilingId.clone()
					x="0"
					y="0"
					width=pattern_width
					height=pattern_height
					patternUnits="userSpaceOnUse"
					viewBox=pattern_view_box
				>
					{grid}
				</pattern>
			</defs>
			<rect fill=format!("url(#{tilingId})") width=width height=height></rect>
		</svg>
	}
}

