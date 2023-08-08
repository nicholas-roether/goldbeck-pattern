use leptos::{ev::MouseEvent, *};

use crate::{
	theme::ThemeCtx,
	tiling::{Shape, Tiling}
};

use super::pattern::{GridColors, Pattern};

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
fn GridLines(
	cx: Scope,
	tiling: Signal<Tiling>,
	width: Signal<f32>,
	height: Signal<f32>
) -> impl IntoView {
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("GridLines is missing theme context!");

	let lines = move || {
		tiling
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
fn Overlay(cx: Scope, tiling: Signal<Tiling>, colors: Signal<GridColors>) -> impl IntoView {
	let view_box =
		move || tiling.with(|t| format!("0 0 {} {}", t.viewport_width(), t.viewport_height()));
	let width = Signal::derive(cx, move || tiling.with(|t| t.viewport_width()));
	let height = Signal::derive(cx, move || tiling.with(|t| t.viewport_height()));

	view! { cx,
		<svg viewBox=view_box class="block outline outline-2 outline-misc shadow-2xl">
			<GridLines tiling width height />
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
pub fn Editor(cx: Scope, tiling: Signal<Tiling>, colors: Signal<GridColors>) -> impl IntoView {
	view! { cx,
		<div class="relative">
			<div class="flex w-[250%] mx-[-75%] md:w-[150%] md:mx-[-25%] xl:w-full xl:mx-0">
				<Pattern tiling colors reps_x=1 reps_y=1 faded=true />
				<Pattern tiling colors reps_x=1 reps_y=1 />
				<Pattern tiling colors reps_x=1 reps_y=1 faded=true />
			</div>
			<div class="absolute inset-0 w-full h-full flex justify-center">
				<Overlay tiling colors />
			</div>
		</div>
	}
}
