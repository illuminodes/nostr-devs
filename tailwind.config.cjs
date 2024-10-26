const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/templates/**/*.html"],
    plugins: [require("@tailwindcss/forms")],
    theme: {
         colors: {
            'nostr-dark': '#4B1862',
            'nostr-light': '#A334D5',
            'black': '#000000',
            'white': '#FFFFFF',
            'transparent': colors.transparent,
        },
        fontFamily: {
            'source': ['SourceCodePro', 'sans-serif'],
        },
    },
}

