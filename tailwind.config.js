import { createThemes } from "tw-colors";

const COLORS = {
  carbon: "#2b323a",
  blue: "#3ebaec",
  white: "#fff",
  gold: "#c1a368",
  aluminium: "#b8bbbe",
};

export default {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      sans: ["Montserrat", "sans-serif"],
    },
  },
  plugins: [
    createThemes({
      light: {
        background: COLORS.white,
        backgroundText: COLORS.carbon,
        primary: COLORS.carbon,
        primaryText: COLORS.white,
        secondary: COLORS.blue,
	    misc: COLORS.aluminium,
	    highlight: COLORS.gold
      },
      dark: {
        background: COLORS.carbon,
        backgroundText: COLORS.white,
        primary: COLORS.white,
        primaryText: COLORS.carbon,
        secondary: COLORS.blue,
	    misc: COLORS.aluminium,
	    highlight: COLORS.gold
      },
    }),
  ],
};
