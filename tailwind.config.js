/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: [
      "./frontend-leptos/**/*.rs",
      "./frontend-leptos/public/**/*.html",
    ],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
