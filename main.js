// tailwind.config.js equivalent for CDN usage
window.tailwind = window.tailwind || {};
tailwind.config = {
    theme: {
        extend: {
            colors: {
                zet: {
                    bg: '#050505',
                    card: '#0F0F12',
                    border: '#27272a',
                    purple: '#a855f7',
                    darkPurple: '#581c87',
                    accent: '#d8b4fe',
                    green: '#22c55e',
                    red: '#ef4444'
                }
            },
            fontFamily: {
                sans: ['Inter', 'sans-serif'],
                mono: ['JetBrains Mono', 'monospace'],
            }
        }
    }
};
