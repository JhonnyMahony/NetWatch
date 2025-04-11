/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
    theme: {
    screens: {
      sm: '576px',
      md: '768px',
      lg: '992px',
      xl: '1200px',
    },
    container: {
      center: true,
      padding: '1rem',
    },
    extend: {
      zIndex: {
        '100': '100',
      },
      colors: {
        primary: '#0275d8'
      },
      fontFamily:{
         'body': [
    'Inter', ],
      }
    },
  },
  variants: {
    extend: {
      visibility: ['group-hover'],
      display: ['group-hover']
    },
  },

  content: ["./src/**/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}
