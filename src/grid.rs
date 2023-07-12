use leptos::*;

use crate::css;

#[derive(Debug, Clone)]
struct Polygon(Vec<(f32, f32)>);

impl Polygon {
	fn points(&self) -> String {
		let mut points = String::new();
		for (x, y) in &self.0 {
			points.push_str(&format!("{x},{y} "))
		}
		points.pop();
		points
	}
}

#[component]
fn Tile<SF>(cx: Scope, shape: SF) -> impl IntoView
where
	SF: Fn() -> Polygon + 'static
{
	let (active, set_active) = create_signal(cx, false);

	let tile = css! {
		&:hover {
			fill: red;
		}
	};

	view! { cx,
		<polygon
			points=move || shape().points()
			fill=move || if active() { "gray" } else { "white" }
			class={tile}
			on:click=move |_| set_active(!active())
		/>
	}
}

const TILE_PATTERN_PERIOD_Y: usize = 8;

fn get_tile_shape(size: f32, x: usize, y: usize) -> Polygon {
	let left_side = x % 2 == 0;
	let svg_x = (x / 2) as f32 * size;
	let svg_y = y as f32 * size;
	let pattern_y = y % TILE_PATTERN_PERIOD_Y;
	let svg_inset_top = (pattern_y as f32 / TILE_PATTERN_PERIOD_Y as f32) * size;
	let svg_inset_bottom = svg_inset_top + size / (TILE_PATTERN_PERIOD_Y as f32);
	if left_side {
		Polygon(vec![
			(svg_x, svg_y),
			(svg_x + size - svg_inset_top, svg_y),
			(svg_x + size - svg_inset_bottom, svg_y + size),
			(svg_x, svg_y + size),
		])
	} else {
		Polygon(vec![
			(svg_x + size - svg_inset_top, svg_y),
			(svg_x + size, svg_y),
			(svg_x + size, svg_y + size),
			(svg_x + size - svg_inset_bottom, svg_y + size),
		])
	}
}

#[component]
pub fn Grid(cx: Scope, size: ReadSignal<usize>) -> impl IntoView {
	let coords_width = move || size() * 2;
	let coords_heigth = size;
	let tile_size = move || 100.0 / size() as f32;

	view! { cx,
		<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
			<defs>
				<filter id="null">
					<feBlend mode="normal" in="SourceGraphic" />
				</filter>
			</defs>
			<g filter="url(#null)">
				<For
					each=move || (0..coords_heigth())
					key=|y| *y
					view=move |cx, y| view! { cx,
						<For
							each=move || (0..coords_width())
							key=|x| *x
							view=move |cx, x| view! { cx,
								<Tile shape=move || get_tile_shape(tile_size(), x, y) />
							}
						/>
					}
				/>
			</g>
		</svg>
	}
}
