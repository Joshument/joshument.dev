/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    './src/**/*.rs',
  ],
  theme: {
    extend: {
      animation: {
        'caret-cursor': 'blink 1s infinite alternate'
      },
      keyframes: {
        blink: {
          '0%': {opacity: '0'},
          '49%': {opacity: '0'},
          '50%': {opacity: '1'},
          '100%': {opacity: '1'}
        }
      },
      transitionProperty: {
        'margins': 'margin-top, margin-bottom'
      }
    },
  },
  plugins: []
}