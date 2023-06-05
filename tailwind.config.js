/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        dark: {
          light: "#333333",
          DEFAULT: "#252526",
          dark: "#1c1c1c",
        },
        gray: {
          dark: "#aaaaaa",
        },
      },
    },
  },
  plugins: [],
};
