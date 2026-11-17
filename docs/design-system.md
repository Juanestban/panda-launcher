# Design system

Black and red, minimalist. Based on the "Visual system" part of the `landing-page-design` skill from [elayadesign/ai-design-skills](https://github.com/elayadesign/ai-design-skills), adapted to a desktop launcher. The launcher (`crates/panda-launcher/src/style.rs`) and the website (`web/`) share these tokens.

## Principles

- Content first: big type, few surfaces, one accent used only for the primary action (Play, Install, Update) and for "you are here".
- No gradient backgrounds, no one-sided borders, no shadows. Separate surfaces with background steps and 1 px borders.
- Every interactive element has hover, active, focus, disabled, loading and error states.
- Skeletons, not spinners, for loading content. Progress bars for downloads.
- Empty states are designed screens with a message and a next action, never a blank area.

## Color

| Token | Value | Use |
|---|---|---|
| `--bg-0` | `#000000` | window, sidebar and page background |
| `--bg-1` | `#181818` | cards, panels, inputs |
| `--bg-2` | `#1F1F1F` | hover surface |
| `--bg-3` | `#272727` | borders, icon wells |
| `--bg-4` | `#313131` | input borders, pressed surface |
| `--text-1` | `#FAFAFA` | primary text |
| `--text-2` | `#9B9B9B` | secondary text |
| `--text-3` | `#6E6E6E` | section labels, hints, disabled |
| `--red` | `#E5484D` | primary action, focus ring, progress, active tab |
| `--red-text` | `#FF6369` | eyebrows, active nav item, errors |
| `--red-tint` | `rgba(229, 72, 77, 0.08–0.16)` | hero card, active nav item, installed build |
| `--red-line` | `rgba(229, 72, 77, 0.28–0.5)` | hero border, hovered card border |
| `--success` | `#3DD68C` | ownership verified, game running |
| `--warning` | `#FFB224` | offline mode, disclaimer |

Backgrounds are the skill's allowed dark set; red is the one accent and the only tint. This deliberately overrides the skill's "flat neutral surfaces only" for the tinted states, at the owner's request (black and red).

## Brand

- Mark: a panda drawn like Bun's logo (soft bun shape, thick black outline, flat cream fill, pink cheeks, red tongue and inner ears). Source `crates/panda-launcher/assets/icon/panda.svg`; PNG and ICO exports sit next to it.
- Used in the sidebar, the Play hero, About, the window and exe icon, the MSI and the website favicon.

## Typography

- Font: **Geist** (UI), **Geist Mono** (versions, hashes, device code). Never Inter, Roboto, Arial, Open Sans or Helvetica.
- No italics, no weight above bold (700).
- Every text uses a named style from the skill (`heading-4`, `title-3`, `body-6`, `caption-5`…), defined once in `style.rs`.
- Eyebrows: 12 px semibold, 1 px tracking, `--red-text`, above each page title.

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
| Hover | surface one step lighter; cards also get `--red-line` borders |
| Active | `scale(0.98)`, surface `--bg-4` |
| Focus | 2 px `--red` ring, offset 2 px, always visible on keyboard focus |
| Disabled | `--text-3`, no hover |
| Loading | skeleton blocks in `--bg-3` with an opacity pulse |
| Error | inline message under the element in `--red-text`, plus a retry action |

## Accessibility

- Text contrast ≥ 4.5:1 against its background (all text tokens pass on `--bg-1`/`--bg-2`).
- Everything is reachable by keyboard; the focus order follows the layout.
- Every icon-only button has an accessible label in both languages.
