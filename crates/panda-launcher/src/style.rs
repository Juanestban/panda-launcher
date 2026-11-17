//! Black and red theme (docs/design-system.md): zgui tokens plus the launcher's own sheet.
//!
//! Text uses the named styles from the design system (heading, title, body, caption) and spacing
//! only the 0/2/4/8/12/16/24/32/40/48/64 scale. Red is the one accent; surfaces stay neutral black.

use zgui_ui_tokens::prelude::*;

/// zgui-ui components read these tokens.
pub fn theme() -> Theme {
  Theme::dark().with_css(
    "--zui-color-background: #000000;
     --zui-color-foreground: #FAFAFA;
     --zui-color-card: #181818;
     --zui-color-card-foreground: #FAFAFA;
     --zui-color-popover: #181818;
     --zui-color-popover-foreground: #FAFAFA;
     --zui-color-muted: #1F1F1F;
     --zui-color-muted-foreground: #9B9B9B;
     --zui-color-secondary: #1F1F1F;
     --zui-color-secondary-foreground: #FAFAFA;
     --zui-color-accent: #272727;
     --zui-color-accent-foreground: #FAFAFA;
     --zui-color-border: #272727;
     --zui-color-input: #313131;
     --zui-color-primary: #E5484D;
     --zui-color-primary-foreground: #FFFFFF;
     --zui-color-ring: #E5484D;
     --zui-color-destructive: #E5484D;
     --zui-color-danger: #E5484D;
     --zui-radius-base: 8px;
     --zui-shadow-sm: none;
     --zui-type-family-sans: 'Geist';
     --zui-type-family-mono: 'Geist Mono';
     --zui-motion-ease-standard: cubic-bezier(0.22, 1, 0.36, 1);",
  )
}

pub const SHEET: &str = zgui::css!(
  ":root { background: #000000; color: #FAFAFA; font-family: 'Geist'; font-size: 14px }

   .heading-4 { font-size: 30px; line-height: 36px; font-weight: 600 }
   .heading-5 { font-size: 24px; line-height: 32px; font-weight: 600 }
   .heading-6 { font-size: 20px; line-height: 28px; font-weight: 600 }
   .title-2 { font-size: 18px; line-height: 28px; font-weight: 500 }
   .title-3 { font-size: 16px; line-height: 24px; font-weight: 500 }
   .title-4 { font-size: 14px; line-height: 20px; font-weight: 500 }
   .body-3 { font-size: 16px; line-height: 24px; font-weight: 400 }
   .body-6 { font-size: 14px; line-height: 20px; font-weight: 400 }
   .caption-3 { font-size: 12px; line-height: 16px; font-weight: 600 }
   .caption-4 { font-size: 12px; line-height: 16px; font-weight: 500 }
   .caption-5 { font-size: 12px; line-height: 16px; font-weight: 400 }
   .muted { color: #9B9B9B }
   .error { color: #FF6369 }
   .success { color: #3DD68C }
   .warning { color: #FFB224 }
   .eyebrow { font-size: 12px; line-height: 16px; font-weight: 600; letter-spacing: 1px; color: #FF6369 }

   .shell { width: 100%; height: 100%; flex-grow: 1; background: #000000 }

   .sidebar { width: 240px; flex-shrink: 0; padding: 24px 16px; gap: 4px; background: #000000 }
   .brand { padding: 0 8px 16px 8px; gap: 12px; align-items: center }
   .brand__logo { width: 40px; height: 40px }
   .brand__text { gap: 2px }
   .brand__name { font-size: 16px; line-height: 24px; font-weight: 600 }
   .nav-label { padding: 16px 12px 8px 12px; font-size: 12px; line-height: 16px; font-weight: 600; letter-spacing: 1px; color: #6E6E6E }

   .nav-item { display: flex; flex-direction: row; padding: 8px 12px; gap: 12px; border-radius: 8px; align-items: center; color: #9B9B9B; cursor: pointer;
               transition: background-color 300ms cubic-bezier(0.22, 1, 0.36, 1), color 300ms cubic-bezier(0.22, 1, 0.36, 1) }
   .nav-item:hover { background: #181818; color: #FAFAFA }
   .nav-item:active { transform: scale(0.98) }
   .nav-item:focus-visible { outline: 2px solid #E5484D; outline-offset: 2px }
   .nav-item.is-active { background: rgba(229, 72, 77, 0.12); color: #FF6369 }

   .icon { width: 20px; height: 20px; flex-shrink: 0 }

   /* Everything clickable shows the hand, and a clearer hover than zgui-ui's defaults on black. */
   .zui-button, .zui-native-select, .zui-switch { cursor: pointer }
   .zui-button--default:hover { background-color: #EC5D61 }
   .zui-button--secondary:hover, .zui-button--outline:hover, .zui-button--ghost:hover { background-color: #272727 }
   .zui-button:disabled, .zui-native-select:disabled, .zui-switch:disabled { cursor: default }
   .account { padding: 12px; gap: 12px; border-radius: 12px; background: #181818; border: 1px solid #272727 }
   .account__row { gap: 12px; align-items: center }
   .account__avatar { width: 32px; height: 32px; border-radius: 8px; background: rgba(229, 72, 77, 0.16); align-items: center;
                      justify-content: center; color: #FF6369; font-weight: 600 }
   .account__text { gap: 2px; flex-grow: 1 }
   .account__status { font-size: 12px; line-height: 16px; color: #3DD68C }
   .account__status.is-offline { color: #FFB224 }
   .account__status.is-out { color: #9B9B9B }

   .main { flex-grow: 1; margin: 8px 8px 8px 0; padding: 40px 48px 0 48px; gap: 24px; border-radius: 16px; background: #000000;
           border: 1px solid #1F1F1F }
   .page-header { gap: 4px }

   .panel { padding: 24px; gap: 16px; border-radius: 16px; background: #181818; border: 1px solid #272727 }
   .field { gap: 8px }
   .field__label { font-size: 12px; line-height: 16px; font-weight: 500; color: #9B9B9B }

   .hero { padding: 32px; gap: 24px; border-radius: 16px; background: rgba(229, 72, 77, 0.08); border: 1px solid rgba(229, 72, 77, 0.28);
           align-items: center }
   .hero__text { flex-grow: 1; gap: 8px }
   .hero__art { width: 144px; height: 144px; flex-shrink: 0 }
   .hero__controls { gap: 12px; align-items: flex-end; padding: 16px 0 0 0 }
   .stats { gap: 12px; align-items: flex-start }
   .stat { padding: 16px; gap: 4px; border-radius: 12px; background: #181818; border: 1px solid #272727; flex-grow: 1; flex-basis: 0 }
   .stat__value { font-size: 20px; line-height: 28px; font-weight: 600 }

   .play-status { gap: 8px }
   .code { font-family: 'Geist Mono'; font-size: 36px; line-height: 40px; font-weight: 600; letter-spacing: 4px; color: #FF6369 }

   .scroll { flex-grow: 1 }
   .stack { gap: 24px; padding: 0 8px 40px 0 }
   .search-row { gap: 8px; align-items: center }
   .search-row__input { flex-grow: 1 }
   .search-row__version, .version-filter__select { width: 128px; flex-shrink: 0 }
   .search-row__version .zui-native-select, .version-filter__select .zui-native-select { width: 100% }
   .version-filter { gap: 16px; align-items: center }
   .version-filter__hint { flex-grow: 1 }
   .skeleton-select { width: 140px; height: 36px; border-radius: 8px }
   .section-head { gap: 8px; align-items: center }
   .section-head__title { flex-grow: 1 }
   .list { gap: 8px }
   .grid { gap: 12px; flex-wrap: wrap }

   .mod-card { display: flex; flex-direction: row; align-items: center; padding: 16px; gap: 16px; border-radius: 12px; background: #181818; border: 1px solid #272727; cursor: pointer;
               transition: background-color 300ms cubic-bezier(0.22, 1, 0.36, 1), border-color 300ms cubic-bezier(0.22, 1, 0.36, 1) }
   .mod-card:hover { background: #1F1F1F; border-color: rgba(229, 72, 77, 0.4) }
   .mod-card:active { transform: scale(0.99) }
   .mod-card:focus-visible { outline: 2px solid #E5484D; outline-offset: 2px }
   .mod-card__body { flex-grow: 1; flex-shrink: 1; flex-basis: 0; gap: 4px }
   .mod-card__title { gap: 8px; align-items: center }
   .mod-card__desc { color: #9B9B9B; height: 20px; overflow: hidden }
   .mod-card__meta { flex-shrink: 0; width: 72px; gap: 4px; align-items: center; color: #9B9B9B }

   .mod-row { display: flex; flex-direction: row; padding: 12px; gap: 12px; border-radius: 12px; background: #181818; border: 1px solid #272727; align-items: center;
              cursor: pointer; transition: background-color 300ms cubic-bezier(0.22, 1, 0.36, 1), border-color 300ms cubic-bezier(0.22, 1, 0.36, 1) }
   .mod-row:hover { background: #1F1F1F; border-color: rgba(229, 72, 77, 0.4) }
   .mod-row:focus-visible { outline: 2px solid #E5484D; outline-offset: 2px }
   .mod-row__text { flex-grow: 1; gap: 2px }

   .mod-icon { border-radius: 8px; background: #272727; align-items: center; justify-content: center; flex-shrink: 0; overflow: hidden;
               color: #6E6E6E }
   .mod-icon--sm { width: 40px; height: 40px }
   .mod-icon--md { width: 48px; height: 48px }
   .mod-icon--lg { width: 96px; height: 96px; border-radius: 16px }
   .mod-icon__img { width: 100%; height: 100% }

   .detail-head { gap: 24px; align-items: center }
   .detail-head__text { flex-grow: 1; gap: 8px }
   .chips { gap: 8px; flex-wrap: wrap }
   .links { gap: 8px; flex-wrap: wrap }
   .tabs { gap: 4px; padding: 4px; border-radius: 12px; background: #181818; border: 1px solid #272727; align-self: flex-start }
   .tab { display: flex; padding: 8px 12px; border-radius: 8px; color: #9B9B9B; font-weight: 500; cursor: pointer;
          transition: background-color 300ms cubic-bezier(0.22, 1, 0.36, 1), color 300ms cubic-bezier(0.22, 1, 0.36, 1) }
   .tab:hover { color: #FAFAFA; background: #1F1F1F }
   .tab.is-active:hover { background: #EC5D61 }
   .tab:focus-visible { outline: 2px solid #E5484D; outline-offset: 2px }
   .tab.is-active { background: #E5484D; color: #FFFFFF }

   .md { gap: 12px; max-width: 760px }
   .md-h1 { font-size: 24px; line-height: 32px; font-weight: 600; padding: 8px 0 0 0 }
   .md-h2 { font-size: 20px; line-height: 28px; font-weight: 600; padding: 8px 0 0 0 }
   .md-h3 { font-size: 16px; line-height: 24px; font-weight: 600 }
   .md-p { font-size: 14px; line-height: 20px; color: #D4D4D4 }
   .md-li { font-size: 14px; line-height: 20px; color: #D4D4D4; padding: 0 0 0 16px }
   .md-code { font-family: 'Geist Mono'; font-size: 12px; line-height: 16px; padding: 12px; border-radius: 8px; background: #000000;
              color: #D4D4D4 }

   .version-row { padding: 12px 16px; gap: 16px; border-radius: 12px; background: #181818; border: 1px solid #272727; align-items: center }
   .version-row.is-installed { border-color: rgba(229, 72, 77, 0.5); background: rgba(229, 72, 77, 0.06) }
   .version-row__text { flex-grow: 1; gap: 2px }

   .update-banner { padding: 12px 16px; gap: 12px; border-radius: 12px; background: rgba(229, 72, 77, 0.12);
                    border: 1px solid rgba(229, 72, 77, 0.4); align-items: center }
   .update-banner__text { flex-grow: 1 }

   .skeleton-row { height: 56px; border-radius: 12px }
   .skeleton-card { height: 82px; border-radius: 12px }
   .settings { gap: 24px; max-width: 640px }
   .settings__toggle { gap: 16px; align-items: center }
   .settings__toggle-text { flex-grow: 1 }
   .about { gap: 16px; max-width: 640px }
   .about__brand { gap: 16px; align-items: center }
   .about__logo { width: 64px; height: 64px }

   @media (prefers-reduced-motion: reduce) {
     .nav-item, .mod-card, .tab { transition: none }
     .nav-item:active, .mod-card:active { transform: none }
   }"
);
