#[macro_export]
macro_rules! css {
	($($css:tt)*) => {{
		use stylist::style;

		let styles = style!($($css)*).expect("Failed to parse CSS styles");
		let class = styles.get_class_name();
		String::from(class)
	}};
}
