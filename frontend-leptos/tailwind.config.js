/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // The Void (Backgrounds) - Deep, Technical Black
                void: {
                    DEFAULT: '#000000', // Pure Void opacity 100
                    surface: '#050505', // Slightly lifted (Cards)
                    overlay: '#0A0A0A', // Modals/Dialogs
                    highlight: '#171717', // Borders/Separators
                    dim: '#030303',     // Secondary background
                },
                // Power Red (Accents) - High Energy, Dangerous
                brand: {
                    light: '#F87171', // Red-400 (Hover text)
                    glow: '#FF0000',  // Pure Red (Neon effects)
                    DEFAULT: '#DC2626', // Red-600 (Primary Action)
                    dark: '#991B1B', // Red-800 (Gradients)
                    deep: '#450a0a', // Red-950 (Background tints)
                    ultra: '#2A0505', // Deepest Red tint
                },
                // Tech Accents
                tech: {
                    chrome: '#E5E7EB', // Metallic text
                    subtle: '#9CA3AF', // Secondary text
                    dark: '#374151',   // Tertiary text
                }
            },
            fontFamily: {
                // Technical, Precise
                sans: ['Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
                // Impactful, Structural
                heading: ['Outfit', 'Inter', 'ui-sans-serif', 'system-ui', 'sans-serif'],
                // Code, Data
                mono: ['JetBrains Mono', 'Fira Code', 'ui-monospace', 'monospace'],
            },
            boxShadow: {
                'glow-red': '0 0 20px -5px rgba(220, 38, 38, 0.5)',
                'glow-red-lg': '0 0 40px -10px rgba(220, 38, 38, 0.6)',
                'glow-red-sm': '0 0 10px -2px rgba(220, 38, 38, 0.4)',
                'neon': '0 0 5px theme("colors.brand.glow"), 0 0 20px theme("colors.brand.glow")',
            },
            backgroundImage: {
                'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
                'cyber-grid': "url(\"data:image/svg+xml,%3Csvg width='40' height='40' viewBox='0 0 40 40' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M0 0h40v40H0V0zm1 1h38v38H1V1z' fill='%23171717' fill-opacity='0.1' fill-rule='evenodd'/%3E%3C/svg%3E\")",
            },
            animation: {
                'float': 'float 6s ease-in-out infinite',
                'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
                'border-beam': 'border-beam 2s linear infinite',
            },
            keyframes: {
                float: {
                    '0%, 100%': { transform: 'translateY(0)' },
                    '50%': { transform: 'translateY(-10px)' },
                },
                'border-beam': {
                    '100%': { transform: 'rotate(360deg)' },
                },
            },
        },
    },
    safelist: [
        'bg-void',
        'bg-void-surface',
        'text-brand',
        'text-brand-glow',
        'font-heading',
        'font-mono',
    ],
    plugins: [],
}
