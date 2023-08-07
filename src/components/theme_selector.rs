use leptos::*;

use crate::theme::{Theme, ThemeCtx};

#[component]
fn ThemeButton(cx: Scope, theme: Theme) -> impl IntoView {
	let theme_ctx = use_context::<ThemeCtx>(cx).expect("ThemeButton is missing theme context!");

	let is_active = move || theme_ctx.get() == theme;
	let dyn_styles = move || {
		if is_active() {
			"shadow-md scale-125"
		} else {
			""
		}
	};

	view! { cx,
		<svg
			role="radio"
			aria-checked=is_active
			tabindex="0"
			aria-label=theme.name()
			width="60px"
			height="60px"
			viewBox="0 0 100 100"
			class=move || format!(
				r#"
                    cursor-pointer rounded-full border-2
					border-misc hover:shadow-lg hover:border-highlight
                    transition-transform
                    {}
                "#,
				dyn_styles()
			)
			on:click=move |_| theme_ctx.set(theme)
			on:keydown=move |_| theme_ctx.set(theme)
		>
			<g class=format!("theme-{}", theme.name())>
				<circle cx="50" cy="50" r="50" class="fill-background" />
				<circle cx="33" cy="62" r="25" class="fill-primary" />
				<circle cx="72" cy="40" r="15" class="fill-secondary" />
			</g>
		</svg>
	}
}

#[component]
pub fn ThemeSelector(cx: Scope) -> impl IntoView {
	view! { cx,
		<div class="py-8 max-w-sm mx-auto flex justify-between">
			<ThemeButton theme=Theme::Bubbles />
			<ThemeButton theme=Theme::Icy />
			<ThemeButton theme=Theme::Glisten />
			<ThemeButton theme=Theme::Noble />
		</div>
	}
}
