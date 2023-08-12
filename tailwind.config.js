function themeColor(name) {
	return `rgb(var(--theme-${name}) / <alpha-value>)`;
}

function importTheme(...names) {
	let colors = {};
	for (let name of names) {
		colors[name] = themeColor(name);
	}
	return colors;
}

export default {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	theme: {
		fontFamily: {
			sans: ["Montserrat", "sans-serif"],
		},
		extend: {
			colors: importTheme(
				"background",
				"backgroundText",
				"primary",
				"primaryText",
				"secondary",
				"secondaryText",
				"misc",
				"highlight"
			)
		}
	},
};
