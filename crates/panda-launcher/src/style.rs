//! The dark theme from docs/design-system.md, as zgui tokens plus the launcher's own sheet.

use zgui_ui_tokens::prelude::*;

/// zgui-ui components read these tokens; values come from the design system table.
pub fn theme() -> Theme {
  Theme::dark().with_css(
    "--zui-color-background: #181818;
     --zui-color-foreground: #FAFAFA;
     --zui-color-card: #1F1F1F;
     --zui-color-card-foreground: #FAFAFA;
     --zui-color-popover: #1F1F1F;
     --zui-color-muted: #272727;
     --zui-color-muted-foreground: #A3A3A3;
     --zui-color-secondary: #272727;
     --zui-color-secondary-foreground: #FAFAFA;
     --zui-color-accent: #272727;
     --zui-color-accent-foreground: #FAFAFA;
     --zui-color-border: #313131;
     --zui-color-input: #313131;
     --zui-color-primary: #4ADE80;
     --zui-color-primary-foreground: #052E16;
     --zui-color-ring: #4ADE80;
     --zui-color-destructive: #F87171;
     --zui-color-danger: #F87171;
     --zui-radius-base: 8px;
     --zui-shadow-sm: none;
     --zui-type-family-sans: 'Geist';
     --zui-type-family-mono: 'Geist Mono';
     --zui-motion-ease-standard: cubic-bezier(0.22, 1, 0.36, 1);",
  )
}

/// Layout and the few surfaces that are not zgui-ui components. Spacing uses the 0/2/4/8/12/16/24/32/40/48 scale only.
pub const SHEET: &str = zgui::css!(
  ":root { background: #000000; color: #FAFAFA; font-family: 'Geist'; font-size: 14px }

   .shell { width: 100%; height: 100%; flex-grow: 1; background: #181818 }

   .sidebar { width: 224px; padding: 24px 16px; gap: 4px; background: #1F1F1F }
   .brand { padding: 0 8px 24px 8px; gap: 8px; align-items: center }
   .brand__name { font-size: 18px; font-weight: 700 }
   .brand__dot { width: 12px; height: 12px; border-radius: 6px; background: #4ADE80 }

   .nav-item { display: flex; flex-direction: row; padding: 8px 12px; gap: 12px; border-radius: 8px; align-items: center; color: #A3A3A3;
               transition: background-color 300ms cubic-bezier(0.22, 1, 0.36, 1) }
   .nav-item:hover { background: #272727; color: #FAFAFA }
   .nav-item:active { transform: scale(0.98) }
   .nav-item:focus-visible { outline: 2px solid #4ADE80; outline-offset: 2px }
   .nav-item.is-active { background: #272727; color: #FAFAFA }

   .icon { width: 20px; height: 20px; flex-shrink: 0 }
   .account { padding: 12px; gap: 8px; border-radius: 12px; background: #272727 }
   .account__name { font-weight: 600 }
   .account__status { font-size: 12px; color: #A3A3A3 }
   .account__status.is-offline { color: #FBBF24 }

   .main { flex-grow: 1; padding: 40px 48px; gap: 24px }
   .page-header { gap: 4px }
   .page-title { font-size: 30px; font-weight: 700 }
   .page-subtitle { color: #A3A3A3 }

   .panel { padding: 24px; gap: 16px; border-radius: 12px; background: #1F1F1F }
   .field { gap: 8px }
   .field__label { font-size: 12px; color: #A3A3A3 }

   .play-row { gap: 16px; align-items: flex-end }
   .play-status { gap: 8px }
   .muted { color: #A3A3A3 }
   .error { color: #F87171 }
   .success { color: #4ADE80 }
   .warning { color: #FBBF24 }

   .code { font-family: 'Geist Mono'; font-size: 36px; font-weight: 700; letter-spacing: 4px; color: #FAFAFA }

   .mods { gap: 24px; flex-grow: 1 }
   .search-row { gap: 8px; align-items: center }
   .search-row__input { flex-grow: 1 }
   .list { gap: 8px }
   .mod-row { padding: 12px 16px; gap: 16px; border-radius: 8px; background: #1F1F1F; align-items: center }
   .mod-row__text { flex-grow: 1; gap: 2px }
   .mod-row__title { font-weight: 600 }
   .mod-row__meta { font-size: 12px; color: #A3A3A3 }
   .section-title { font-size: 18px; font-weight: 600 }

   .skeleton-row { height: 56px; border-radius: 8px }

   .settings { gap: 24px; max-width: 640px }
   .settings__toggle { gap: 16px; align-items: center }
   .settings__toggle-text { flex-grow: 1 }

   .about { gap: 12px; max-width: 640px }

   @media (prefers-reduced-motion: reduce) {
     .nav-item { transition: none }
     .nav-item:active { transform: none }
   }"
);
