use leptos::{
	component, create_effect, create_rw_signal, provide_context, Children, IntoView, RwSignal,
};

use web_sys::{window, CssStyleDeclaration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Theme {
	Bubbles,
	Icy,
	Glisten,
	Noble
}

impl Theme {
	pub fn name(self) -> &'static str {
		match self {
			Self::Bubbles => "bubbles",
			Self::Icy => "icy",
			Self::Glisten => "glisten",
			Self::Noble => "noble"
		}
	}
}

impl Default for Theme {
	fn default() -> Self {
		Self::Bubbles
	}
}

#[derive(Debug)]
pub struct ThemeData {
	pub background: String,
	pub primary: String,
	pub secondary: String,
	pub misc: String,
	pub highlight: String
}

impl ThemeData {
	pub fn load() -> Self {
		let computed_styles = Self::computed_styles();
		let background = Self::color(&computed_styles, "background");
		let primary = Self::color(&computed_styles, "primary");
		let secondary = Self::color(&computed_styles, "secondary");
		let misc = Self::color(&computed_styles, "misc");
		let highlight = Self::color(&computed_styles, "highlight");

		Self {
			background,
			primary,
			secondary,
			misc,
			highlight
		}
	}

	fn computed_styles() -> CssStyleDeclaration {
		let window = window().unwrap();
		let root = window.document().unwrap().document_element().unwrap();
		window.get_computed_style(&root).unwrap().unwrap()
	}

	fn get_property(computed_styles: &CssStyleDeclaration, name: &str) -> String {
		computed_styles.get_property_value(name).unwrap()
	}

	fn color(computed_styles: &CssStyleDeclaration, name: &'static str) -> String {
		let var_name = format!("--theme-{name}");
		let var_value = Self::get_property(computed_styles, &var_name);
		let components = var_value.split(' ').collect::<Vec<&str>>();
		format!(
			"rgb({}, {}, {})",
			components[0], components[1], components[2]
		)
	}
}

pub type ThemeCtx = RwSignal<Theme>;

#[component]
pub fn ThemeManager(children: Children) -> impl IntoView {
	let theme = create_rw_signal(Theme::default());
	create_effect(move |_| {
		let root = leptos::document().document_element().unwrap();
		root.set_attribute("data-theme", theme().name()).unwrap();
	});

	provide_context::<ThemeCtx>(theme);

	children()
}
