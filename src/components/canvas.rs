use std::rc::Rc;

use leptos::{
	ev::{MouseEvent, TouchEvent},
	leptos_dom::logging::console_log,
	*
};
use web_sys::{MouseEventInit, SvgElement};

use crate::{
	components::pattern::{GridColors, Pattern, TileColor},
	tiling::{Shape, Tiling}
};

#[component]
fn TileOverlay(
	shape: Shape,
	color: RwSignal<TileColor>,
	brush: Signal<TileColor>
) -> impl IntoView {
	let paint = move || color.set(brush.get_untracked());

	let handle_mouse = move |buttons: u16| {
		if buttons & 0b01 == 1 {
			paint();
		}
	};

	let on_mouse_enter = move |evt: MouseEvent| {
		let target: SvgElement = event_target(&evt);
		target.focus().unwrap_or(());
		handle_mouse(evt.buttons());
	};

	let on_mouse_down = move |evt: MouseEvent| {
		handle_mouse(evt.buttons());
	};

	let on_touch_start = move |_: TouchEvent| {
		paint();
	};

	view! {
		<polygon
			class="cursor-crosshair hover:stroke-highlight"
			points=shape.svg_path()
			vector-effect="non-scaling-stroke"
			fill="transparent"
			stroke-width="4"
			stroke-linejoin="round"
			on:mouseenter=on_mouse_enter
			on:mousedown=on_mouse_down
			on:touchstart=on_touch_start
		/>
	}
}

#[component]
fn GridLines(tiling: Signal<Rc<Tiling>>, width: Signal<f32>, height: Signal<f32>) -> impl IntoView {
	let lines = move || {
		tiling
			.with(|t| t.iter_lines())
			.map(|[p1, p2]| {
				view! {
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
			.collect_view()
	};

	view! {
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
	tiling: Signal<Rc<Tiling>>,
	colors: Signal<GridColors>,
	brush: Signal<TileColor>
) -> impl IntoView {
	let view_box =
		move || tiling.with(|t| format!("0 0 {} {}", t.viewport_width(), t.viewport_height()));
	let width = Signal::derive(move || tiling.with(|t| t.viewport_width()));
	let height = Signal::derive(move || tiling.with(|t| t.viewport_height()));

	let on_touch_move = |evt: TouchEvent| {
		if evt.touches().length() != 1 {
			return;
		};
		evt.prevent_default();
		let touch = evt.touches().item(0).unwrap();
		let Some(elem) =
			document().element_from_point(touch.page_x() as f32, touch.page_y() as f32)
		else {
			return;
		};
		console_log(&elem.tag_name());
		elem.dispatch_event(
			&MouseEvent::new_with_mouse_event_init_dict(
				"mouseenter",
				MouseEventInit::new().buttons(0b01)
			)
			.unwrap()
		)
		.expect("Failed to dispatch mouseenter event");
	};

	view! {
		<svg
			viewBox=view_box
			width="100%"
			class="block touch-none"
			on:touchmove=on_touch_move
		>
			<GridLines tiling width height />
			{move || tiling
				.with(|t| t.iter_tiles())
				.enumerate()
				.map(|(i, shape)| {
					view! {
						<TileOverlay shape color=colors.with(|c| c.get_color(i)) brush />
					}
				})
				.collect_view()
			}
		</svg>
	}
}

#[component]
pub fn Canvas(
	#[prop(into)] tiling: Signal<Rc<Tiling>>,
	#[prop(into)] colors: Signal<GridColors>,
	brush: RwSignal<TileColor>
) -> impl IntoView {
	let aspect_ratio = move || {
		tiling
			.with(|t| t.viewport_width() / t.viewport_height())
			.to_string()
	};
	view! {
		<div class="relative h-full max-w-full m-auto" style:aspect-ratio=aspect_ratio>
			<div class="absolute flex inset-0 mx-[-100%] h-full z-0">
				<Pattern id="canvas" tiling colors reps_x=3 reps_y=1 />
				<div class="absolute flex inset-0 w-full h-full">
					<div class="flex-1 bg-background/50 transition-colors" />
					<div class="flex-1 bg-transparent" />
					<div class="flex-1 bg-background/50 transition-colors" />
				</div>
			</div>
			<div class="relative z-1 w-full outline outline-2 outline-misc shadow-2xl ">
				<Overlay tiling colors brush=brush.into() />
			</div>
		</div>
	}
}
