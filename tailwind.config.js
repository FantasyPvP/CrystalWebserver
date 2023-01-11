/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/*.{html,rs}"],
  theme: {
    extend: {
      screens: {
        '3xl': '2250px'
        // => @media (min-width: 1000px) { ... }
      },
    },
  },
  plugins: [],
}
