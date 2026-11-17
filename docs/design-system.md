# Design system

Dark and minimalist. Based on the "Visual system" part of the `landing-page-design` skill from [elayadesign/ai-design-skills](https://github.com/elayadesign/ai-design-skills), adapted to a desktop launcher. The launcher (zgui CSS variables) and the website (`web/`) share these tokens.

## Principles

- Content first: big type, few surfaces, one accent used only for the primary action (Play, Install, Update).
- No gradient backgrounds, no one-sided borders, no shadows. Separate surfaces with background steps and 1 px borders.
- Every interactive element has hover, active, focus, disabled, loading and error states.
- Skeletons, not spinners, for loading content. Progress bars for downloads.
- Empty states are designed screens with a message and a next action, never a blank area.

## Color

| Token | Value | Use |
|---|---|---|
| `--bg-0` | `#000000` | window behind everything |
| `--bg-1` | `#181818` | app background |
| `--bg-2` | `#1F1F1F` | cards, sidebar |
| `--bg-3` | `#272727` | inputs, hover surface |
| `--bg-4` | `#313131` | pressed surface, borders |
| `--text-1` | `#FAFAFA` | primary text |
| `--text-2` | `#A3A3A3` | secondary text |
| `--text-3` | `#737373` | hints, disabled |
| `--accent` | `#4ADE80` | primary action, focus ring, progress |
| `--accent-text` | `#052E16` | text on accent |
| `--danger` | `#F87171` | errors, destructive actions |
| `--warning` | `#FBBF24` | warnings (incompatible mod, offline mode) |

The background ramp is the skill's allowed dark set. Bamboo green is the single accent.

## Typography

- Font: **Geist** (UI), **Geist Mono** (versions, hashes, device code). Never Inter, Roboto, Arial, Open Sans or Helvetica.
- No italics, no weight above bold (700).
- Scale (Tailwind steps, px): 12 caption · 14 body-sm · 16 body · 18 title-sm · 20 title · 24 h3 · 30 h2 · 36 h1.

## Spacing and radius

- Spacing scale (px): 0, 2, 4, 8, 12, 16, 24, 32, 40, 48, 64, 80, 96. Nothing else.
- Primary button padding: 8 vertical, 12 horizontal (more for the large Play button: 12 × 24).
- Radius (Tailwind): 4 sm, 6 md, 8 lg, 12 xl. Nested: inner = outer − gap (when the gap is < 32 and the result is > 2).

## Icons

Phosphor icons only, regular weight, 16 or 20 px. Never Material icons.

## Motion

- Easing `cubic-bezier(0.22, 1, 0.36, 1)`; 300 ms for hover/press, 500 ms for layout.
- Animate only `transform` and `opacity`. Active: `scale(0.98)`.
- Respect the OS reduced-motion setting by disabling non-essential motion.

## States

| State | Rule |
|---|---|
| Hover | surface one step lighter (`--bg-n+1`) |
| Active | `scale(0.98)`, surface `--bg-4` |
| Focus | 2 px `--accent` ring, offset 2 px, always visible on keyboard focus |
| Disabled | `--text-3`, no hover |
| Loading | skeleton blocks in `--bg-3` with an opacity pulse |
| Error | inline message under the element in `--danger`, plus a retry action |

## Accessibility

- Text contrast ≥ 4.5:1 against its background (all text tokens pass on `--bg-1`/`--bg-2`).
- Everything is reachable by keyboard; the focus order follows the layout.
- Every icon-only button has an accessible label in both languages.
