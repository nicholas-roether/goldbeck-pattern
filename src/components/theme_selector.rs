use leptos::*;
use web_sys::KeyboardEvent;

use crate::{
	cls,
	theme::{Theme, ThemeCtx}
};

#[component]
fn ThemeButton(theme: Theme) -> impl IntoView {
	let theme_ctx = use_context::<ThemeCtx>().expect("ThemeButton is missing theme context!");

	let is_active = move || theme_ctx() == theme;

	let on_keydown = move |evt: KeyboardEvent| {
		if &evt.code() == "Enter" {
			evt.prevent_default();
			theme_ctx.set(theme);
		}
	};

	view! {
		<div
			role="radio"
			aria-checked=is_active
			tabindex="0"
			aria-label=theme.name()
			class=move || cls! {
				"rounded-full transition-all w-10 h-10 sm:w-12 sm:h-12 outline overflow-hidden",
				if is_active() {
					"shadow-2xl outline-3 outline-highlight"
				} else {
					"cursor-pointer outline-2 outline-misc hover:shadow-md"
				}
			}
			on:click=move |_| theme_ctx.set(theme)
			on:keydown=on_keydown
		 >
			<svg
				viewBox="0 0 100 100"
				class="w-full h-full bg-background"
				data-theme=theme.name()
			>
				<circle cx="33" cy="62" r="25" class="fill-primary" />
				<circle cx="72" cy="40" r="15" class="fill-secondary" />
			</svg>
		</div>
	}
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
	view! {
		<div
			role="radiogroup"
			aria-label="Farbschema"
			class="p-3 sm:p-6 w-full max-w-sm mx-auto flex justify-between"
		>
			<ThemeButton theme=Theme::Bubbles />
			<ThemeButton theme=Theme::Icy />
			<ThemeButton theme=Theme::Glisten />
			<ThemeButton theme=Theme::Noble />
		</div>
	}
}
