use leptos::{
	component, create_effect, create_memo, create_rw_signal, provide_context, Children, IntoView,
	Memo, RwSignal, Scope, SignalSet, SignalWith
};
use web_sys::{window, CssStyleDeclaration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Theme {
	Light,
	Dark
}

impl Theme {
	fn name(self) -> &'static str {
		match self {
			Self::Light => "light",
			Self::Dark => "dark"
		}
	}
}

impl Default for Theme {
	fn default() -> Self {
		Self::Light
	}
}

#[derive(Debug, Clone)]
pub struct ThemeCtx {
	pub background: Memo<String>,
	pub primary: Memo<String>,
	pub secondary: Memo<String>,
	pub misc: Memo<String>,
	pub highlight: Memo<String>,
	theme: RwSignal<Theme>
}

impl ThemeCtx {
	pub fn set(&self, theme: Theme) {
		self.theme.set(theme);
	}

	fn new(cx: Scope, theme: RwSignal<Theme>) -> Self {
		let computed_styles = Self::computed_styles(cx, theme);
		let background = Self::color(cx, computed_styles, "background");
		let primary = Self::color(cx, computed_styles, "primary");
		let secondary = Self::color(cx, computed_styles, "secondary");
		let misc = Self::color(cx, computed_styles, "misc");
		let highlight = Self::color(cx, computed_styles, "highlight");

		Self {
			background,
			primary,
			secondary,
			theme,
			misc,
			highlight
		}
	}

	fn computed_styles(cx: Scope, theme: RwSignal<Theme>) -> Memo<CssStyleDeclaration> {
		let window = window().unwrap();
		let root = window.document().unwrap().document_element().unwrap();
		create_memo(cx, move |_| {
			theme.with(|_| ());
			window.get_computed_style(&root).unwrap().unwrap()
		})
	}

	fn get_property(computed_styles: Memo<CssStyleDeclaration>, name: &str) -> String {
		let styles = computed_styles();
		styles.get_property_value(name).unwrap()
	}

	fn color(
		cx: Scope,
		computed_styles: Memo<CssStyleDeclaration>,
		name: &'static str
	) -> Memo<String> {
		create_memo(cx, move |_| {
			let var_name = format!("--twc-{name}");
			let components = Self::get_property(computed_styles, &var_name);
			format!("hsl({components})")
		})
	}
}

#[component]
pub fn ThemeManager(cx: Scope, children: Children) -> impl IntoView {
	let theme = create_rw_signal(cx, Theme::default());
	create_effect(cx, move |_| {
		let root = leptos::document().document_element().unwrap();
		root.set_attribute("data-theme", theme().name()).unwrap();
	});

	let theme_ctx = ThemeCtx::new(cx, theme);
	provide_context::<ThemeCtx>(cx, theme_ctx);

	children(cx)
}
