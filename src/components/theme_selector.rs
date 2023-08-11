use leptos::*;
use web_sys::KeyboardEvent;

use crate::theme::{Theme, ThemeCtx};

#[component]
fn ThemeButton(cx: Scope, theme: Theme) -> impl IntoView {
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("ThemeButton is missing theme context!");

	let is_active = move || theme_ctx() == theme;
	let dyn_styles = move || {
		if is_active() {
			"shadow-lg scale-125"
		} else {
			"cursor-pointer hover:shadow-md hover:border-highlight"
		}
	};

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
			width="50px"
			height="50px"
			viewBox="0 0 100 100"
			class=move || format!(
				"rounded-full border-2 border-misc transition-all {}",
				dyn_styles()
			)
			on:click=move |_| theme_ctx.set(theme)
			on:keydown=on_keydown
		>
			<g class=format!("theme-{}", theme.name())>
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
