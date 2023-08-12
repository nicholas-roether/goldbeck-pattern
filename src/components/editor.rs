use leptos::{ev::MouseEvent, *};

use crate::{
	components::pattern::TileColor,
	tiling::{Shape, Tiling}
};

use super::pattern::{GridColors, Pattern};

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

	let class = move || {
		if hovering() {
			"cursor-crosshair stroke-highlight"
		} else {
			"cursor-crosshair"
		}
	};

	view! { cx,
		<polygon
			class=class
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
	tiling: Signal<Tiling>,
	colors: Signal<GridColors>,
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

// #[component]
// fn BrushButton(cx: Scope, color: TileColor, brush: RwSignal<TileColor>) -> impl IntoView {
// 	let bg_class = match color {
// 		TileColor::Primary => "bg-primary text-primaryText before:bg-primary after:bg-primary",
// 		TileColor::Secondary => {
// 			"bg-secondary text-secondaryText before:bg-secondary after:bg-primary"
// 		}
// 		TileColor::None => "bg-background text-backgroundText before:bg-background after:bg-primary"
// 	};
// 	let active_class = move || {
// 		if brush() == color {
// 			"todo"
// 		} else {
// 			""
// 		}
// 	};
//
// 	let class = move || {
// 		format!(
// 			r#"
//                 transition-all h-20 flex-1 flex items-center justify-center
//                 before:content-[""] before:block before:w-4 before:absolute before:-left-3
//                 before:bor
//                 {bg_class} {}
//             "#,
// 			active_class()
// 		)
// 	};
//
// 	view! { cx,
// 		<button
// 			class=class
// 			role="radio"
// 			aria-label=match color {
// 				TileColor::Primary => "Pinsel 1",
// 				TileColor::Secondary => "Pinsel 2",
// 				TileColor::None => "Radiergummi"
// 			}
// 			on:click=move |_| brush.set(color)
// 		>
// 			{if color == TileColor::None {
// 				view! { cx,
// 					<box-icon name="eraser" size="md" color="currentColor" />
// 				}
// 			} else {
// 				view! { cx,
// 					<box-icon name="brush" size="md" color="currentColor" />
// 				}
// 			}}
// 		</button>
// 	}
// }

#[component]
fn BrushButton(
	cx: Scope,
	name: &'static str,
	icon: &'static str,
	color: TileColor,
	brush: RwSignal<TileColor>
) -> impl IntoView {
	view! { cx,
		<button
			role="radio"
			aria-label=name
			class=move || String::new()
				+ "inline-flex p-2 "
				+ match color {
					TileColor::Primary => "bg-primary text-primaryText ",
					TileColor::Secondary => "bg-secondary text-secondaryText ",
					TileColor::None => "bg-background text-backgroundText "
				}
				+ if brush() == color { "relative z-100 outline outline-3 outline-highlight " } else { " " }
			on:click=move |_| brush.set(color)
		>
			<box-icon name=icon size="md" color="currentColor" />
		</button>
	}
}

#[component]
fn BrushControls(cx: Scope, brush: RwSignal<TileColor>) -> impl IntoView {
	view! { cx,
		<div role="radiogroup" aria-label="Werkzeug auswählen" class="flex w-full max-w-sm border-2 border-misc">
			<BrushButton name="Pinsel 1" icon="brush" color=TileColor::Primary brush />
			<BrushButton name="Pinsel 2" icon="brush" color=TileColor::Secondary brush />
			<BrushButton name="Radiergummi" icon="eraser" color=TileColor::None brush />
		</div>
	}
}

#[component]
fn Controls(cx: Scope, brush: RwSignal<TileColor>) -> impl IntoView {
	view! { cx,
		<div class="p-6">
			<BrushControls brush />
		</div>
	}
}

#[component]
pub fn Editor(cx: Scope, tiling: Signal<Tiling>, colors: Signal<GridColors>) -> impl IntoView {
	let brush = create_rw_signal(cx, TileColor::Primary);
	view! { cx,
		<div class="px-4 sm:px-16 flex flex-col items-center flex-1 w-full min-h-0">
			<div class="shrink w-full min-h-0">
				<Canvas tiling colors brush />
			</div>
			<Controls brush />
		</div>
	}
}
