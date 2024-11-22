/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      borderColor: {
        DEFAULT: "#3f3f46",
      },
    },
  },
  plugins: [],
};
