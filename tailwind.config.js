/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./crates/front/src/**/*.rs",
    "./crates/front/src/*.rs",
    "./crates/front/src/*.rs",
    './node_modules/preline/dist/*.js',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('preline/plugin'),
  ],
}
