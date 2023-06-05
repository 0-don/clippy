/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        main: "#238C3D",
        mainAlt: "#29A649",
        mainAlt2: "#175927",
        container: "#16191E",
        containerAlt: "#1A1D22",
        card: "#171717",
        inputIconBg: "#1E1E22",
        inputBg: "#16191E",
        inputPlaceholder: "#D5D5D5",
        inputBord: "#464645",
        submitButton: "#e5e4e2",
        submitAltButton: "#030303",
        bodyBg: "#1f2025",
      },
    },
  },
  plugins: [],
};
