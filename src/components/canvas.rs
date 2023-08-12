use leptos::{ev::MouseEvent, *};

use crate::{
	cls,
	components::pattern::{GridColors, Pattern, TileColor},
	tiling::{Shape, Tiling}
};

#[component]
fn TileOverlay(
	cx: Scope,
	shape: Shape,
	color: RwSignal<TileColor>,
	brush: Signal<TileColor>
) -> impl IntoView {
	let (hovering, set_hovering) = create_signal(cx, false);

	let handle_mouse = move |buttons: u16| {
		let new_color = brush.get_untracked();
		if buttons & 0x01 == 1 {
			color.set(new_color);
		}
	};

	let on_mouse_enter = move |evt: MouseEvent| {
		set_hovering(true);
		handle_mouse(evt.buttons());
	};

	let on_mouse_leave = move |_: MouseEvent| {
		set_hovering(false);
	};

	let on_mouse_down = move |evt: MouseEvent| {
		handle_mouse(evt.buttons());
	};

	view! { cx,
		<polygon
			class=move || cls! {
				if hovering() {
					"cursor-crosshair stroke-highlight"
				} else {
					"cursor-crosshair"
				}
			}
			points=shape.svg_path()
			vector-effect="non-scaling-stroke"
			fill="transparent"
			stroke-width="4"
			stroke-linejoin="round"
			on:mouseenter=on_mouse_enter
			on:mouseleave=on_mouse_leave
			on:mousedown=on_mouse_down
		/>
	}
}

#[component]
fn GridLines(
	cx: Scope,
	tiling: Signal<Tiling>,
	width: Signal<f32>,
	height: Signal<f32>
) -> impl IntoView {
	let lines = move || {
		tiling
			.with(|t| t.iter_lines())
			.map(|[p1, p2]| {
				view! { cx,
					<line
						class="stroke-misc"
						x1=p1.x
						y1=p1.y
						x2=p2.x
						y2=p2.y
						stroke-width="1"
						vector-effect="non-scaling-stroke"
					/>
				}
			})
			.collect_view(cx)
	};

	view! { cx,
		<defs>
			<clipPath id="GridLines__centerSquare">
				<rect x="0" y="0" width=width height=height />
			</clipPath>
		</defs>
		<g clip-path="url(#centerSquare)">{lines}</g>
	}
}

#[component]
fn Overlay(
	cx: Scope,
	tiling: Signal<Tiling>,
	colors: Signal<GridColors>,
	brush: Signal<TileColor>
) -> impl IntoView {
	let view_box =
		move || tiling.with(|t| format!("0 0 {} {}", t.viewport_width(), t.viewport_height()));
	let width = Signal::derive(cx, move || tiling.with(|t| t.viewport_width()));
	let height = Signal::derive(cx, move || tiling.with(|t| t.viewport_height()));

	view! { cx,
		<svg viewBox=view_box width="100%" class="block outline outline-2 outline-misc shadow-2xl">
			<GridLines tiling width height />
			{move || tiling
				.with(|t| t.iter_tiles())
				.enumerate()
				.map(|(i, shape)| {
					view! { cx,
						<TileOverlay shape color=colors.with(|c| c.get_color(i)) brush />
					}
				})
				.collect_view(cx)
			}
		</svg>
	}
}

#[component]
pub fn Canvas(
	cx: Scope,
	#[prop(into)] tiling: Signal<Tiling>,
	#[prop(into)] colors: Signal<GridColors>,
	brush: RwSignal<TileColor>
) -> impl IntoView {
	let aspect_ratio = move || {
		tiling
			.with(|t| t.viewport_width() / t.viewport_height())
			.to_string()
	};
	view! { cx,
		<div class="relative h-full max-w-full m-auto" style:aspect-ratio=aspect_ratio>
			<div class="absolute flex inset-0 mx-[-100%] h-full z-0">
				<Pattern tiling colors reps_x=1 reps_y=1 background=true />
				<Pattern tiling colors reps_x=1 reps_y=1 />
				<Pattern tiling colors reps_x=1 reps_y=1 background=true />
			</div>
			<div class="relative z-1 w-full">
				<Overlay tiling colors brush=brush.into() />
			</div>
		</div>
	}
}
