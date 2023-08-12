use leptos::*;
use web_sys::KeyboardEvent;

use crate::{
	cls,
	theme::{Theme, ThemeCtx}
};

#[component]
fn ThemeButton(cx: Scope, theme: Theme) -> impl IntoView {
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("ThemeButton is missing theme context!");

	let is_active = move || theme_ctx() == theme;

	let on_keydown = move |evt: KeyboardEvent| {
		if &evt.code() == "Enter" {
			evt.prevent_default();
			theme_ctx.set(theme);
		}
	};

	view! { cx,
		<svg
			role="radio"
			aria-checked=is_active
			tabindex="0"
			aria-label=theme.name()
			width="50"
			height="50"
			viewBox="0 0 100 100"
			class=move || cls! {
				"rounded-full transition-all",
				if is_active() {
					"shadow-lg scale-125 border-2 border-highlight"
				} else {
					"cursor-pointer border-2 border-misc hover:shadow-md"
				}
			}
			on:click=move |_| theme_ctx.set(theme)
			on:keydown=on_keydown
		>
			<g data-theme=theme.name()>
				<rect x="0" y="0" width="100" height="100" class="fill-background" />
				<circle cx="33" cy="62" r="25" class="fill-primary" />
				<circle cx="72" cy="40" r="15" class="fill-secondary" />
			</g>
		</svg>
	}
}

#[component]
pub fn ThemeSelector(cx: Scope) -> impl IntoView {
	view! { cx,
		<div
			role="radiogroup"
			aria-label="Farbschema"
			class="p-6 w-full max-w-sm mx-auto flex justify-between"
		>
			<ThemeButton theme=Theme::Bubbles />
			<ThemeButton theme=Theme::Icy />
			<ThemeButton theme=Theme::Glisten />
			<ThemeButton theme=Theme::Noble />
		</div>
	}
}
