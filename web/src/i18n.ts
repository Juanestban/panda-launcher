// Every string on the site, in Spanish and English. Copy rules: sentence case, no hyphens
// inside sentences, no invented numbers (docs/design-system.md, landing-page-design skill).

export const langs = ["es", "en"] as const;
export type Lang = (typeof langs)[number];

const es = {
  htmlTitle: "Panda Launcher · Mods de Minecraft ordenados por versión",
  metaDescription:
    "Launcher de Minecraft Java para Windows. Inicia sesión con Microsoft, instala mods de Modrinth con un clic y cada versión guarda sus propios mods.",
  skip: "Saltar al contenido",
  nav: { features: "Funciones", how: "Cómo funciona", faq: "Preguntas", download: "Descargar", menu: "Abrir menú", close: "Cerrar menú", other: "English", otherLabel: "Read in English" },

  heroEyebrow: "Minecraft Java · Fabric · Windows",
  heroTitle: ["Tus mods en su versión.", "Sin mover una carpeta."],
  heroText:
    "Inicia sesión con tu cuenta de Microsoft, elige la versión e instala mods de Modrinth con un clic. Cada versión guarda sus propios mods, así que cambiar de una a otra no rompe nada.",
  cta: "Descargar para Windows",
  ctaSource: "Ver el código",
  proof: "Gratis y de código abierto · MIT o Apache 2.0 · Hecho en Rust",
  shotPlay: "La pantalla Jugar de Panda Launcher, con el panda y el botón para iniciar sesión",

  featuresEyebrow: "Funciones",
  featuresTitle: ["Hecho para quien juega", "muchas versiones con mods."],
  features: {
    folders: {
      title: "Una carpeta por versión",
      text: "1.21.10 y 26.3 nunca comparten mods. El launcher arranca cada versión con su carpeta, así que no hay que mover archivos para cambiar.",
    },
    modrinth: {
      title: "Modrinth dentro del launcher",
      text: "Busca, lee la descripción y elige la build exacta. Las dependencias se instalan solas.",
    },
    legal: {
      title: "Legal, con tu cuenta",
      text: "Inicias sesión con Microsoft y el launcher comprueba que tienes el juego. No existe un modo pirata.",
    },
    secrets: {
      title: "Tu sesión no sale del sistema",
      text: "Los tokens se guardan en el Administrador de credenciales de Windows, nunca en archivos ni registros.",
    },
    updates: {
      title: "Se actualiza solo, con firma",
      text: "Cada versión nueva se comprueba con una firma ed25519 y su SHA 256 antes de instalarse.",
    },
  },
  treeCaption: "Así quedan tus mods en disco",

  tagline: ["Elige la versión.", "Instala lo que quieras.", "Pulsa jugar."],

  howEyebrow: "Cómo funciona",
  howTitle: ["De la descarga a la partida", "en tres pasos."],
  steps: [
    { title: "Inicia sesión", text: "Abre el enlace de Microsoft, escribe el código que te muestra el launcher y listo. Se comprueba que tienes Minecraft.", alt: "Pantalla Jugar con el botón para iniciar sesión con Microsoft" },
    { title: "Busca mods", text: "La pestaña Mods muestra lo más popular de Modrinth para tu versión, con su icono y sus descargas.", alt: "Pantalla Mods con tarjetas de Fabric API, Sodium, Iris Shaders y otros mods" },
    { title: "Elige la build y juega", text: "Cada mod tiene su página con descripción, enlaces y todas las builds de Fabric para tu versión.", alt: "Página de Sodium dentro del launcher con estadísticas, enlaces y pestañas" },
  ],

  faqEyebrow: "Preguntas",
  faqTitle: "Lo que suelen preguntar",
  faq: [
    { q: "¿Necesito comprar Minecraft?", a: "Sí. Panda Launcher usa tu cuenta de Microsoft y comprueba que tienes Minecraft Java Edition antes de jugar. No hay cuentas sin compra." },
    { q: "¿Es un producto oficial de Minecraft?", a: "No. No está aprobado ni asociado con Mojang o Microsoft. Es un proyecto de código abierto." },
    { q: "¿Dónde se guardan mis mods?", a: "Cada versión tiene su carpeta, por ejemplo profiles/1.21.10-fabric/mods, dentro de los datos del launcher. Puedes abrirla y verlos como archivos normales." },
    { q: "¿Funciona con Forge o NeoForge?", a: "No. El launcher usa solo Fabric, y los mods vienen solo de Modrinth. Así cada instalación es predecible." },
    { q: "¿Puedo jugar sin internet?", a: "Sí, con una cuenta que ya se verificó en línea antes en este equipo. La primera vez hace falta conexión." },
    { q: "¿Instalador o portable?", a: "El instalador MSI se instala solo para tu usuario, sin permisos de administrador, y aparece en el menú Inicio. El portable es un único archivo .exe. Los dos se actualizan solos." },
    { q: "¿Qué datos envía el launcher?", a: "Solo lo necesario para funcionar: inicio de sesión con Microsoft, descargas de Mojang, Fabric y Modrinth, y la comprobación de versiones en GitHub. No hay telemetría ni analítica." },
    { q: "¿Hay versión para macOS o Linux?", a: "Por ahora solo publicamos builds para Windows. El código es Rust multiplataforma, así que es posible más adelante." },
  ],

  finalTitle: ["Instala tus mods una vez.", "Juega en cualquier versión."],
  finalText: "Gratis, de código abierto y sin cuentas extra.",
  version: (v: string, date: string) => `Versión ${v} · ${date}`,
  portable: (mb: string) => `o descarga el portable (${mb} MB)`,
  msiSize: (mb: string) => `Instalador MSI · ${mb} MB`,
  noRelease: "Todavía no hay una versión publicada. Puedes seguir el progreso en GitHub.",
  releases: "Todas las versiones",

  footer: {
    disclaimer: "No es un producto oficial de Minecraft. No está aprobado ni asociado con Mojang o Microsoft.",
    privacy: "Privacidad",
    license: "Licencia",
    source: "Código fuente",
  },

  privacy: {
    title: "Privacidad",
    description: "Qué datos usa Panda Launcher y con quién habla.",
    updated: "Actualizado el 17 de noviembre de 2026",
    sections: [
      { h: "El launcher", p: ["Panda Launcher no tiene servidores propios ni recoge telemetría. Habla solo con estos servicios, y solo para lo que necesita:", "Microsoft y Mojang para iniciar sesión, comprobar que tienes el juego y descargarlo. Fabric para el loader. Modrinth para buscar e instalar mods. GitHub para buscar actualizaciones.", "Tu correo y tus datos personales nunca se envían en ninguna petición. Las peticiones se identifican solo con el nombre del launcher y la dirección de su repositorio."] },
      { h: "Tu cuenta", p: ["Los tokens de tu sesión se guardan en el Administrador de credenciales de Windows. Nunca se escriben en archivos ni en registros. Al cerrar sesión se borran."] },
      { h: "Esta web", p: ["Esta página no usa cookies, analítica ni scripts de terceros. La aloja GitHub Pages, que puede registrar datos técnicos de las visitas según su propia política."] },
      { h: "Contacto", p: ["Para cualquier duda, abre un issue en el repositorio de GitHub."] },
    ],
    back: "Volver al inicio",
  },
  notFound: { title: "Esta página no existe", text: "Puede que el enlace esté roto o que la página se haya movido.", back: "Volver al inicio" },
};

const en: typeof es = {
  htmlTitle: "Panda Launcher · Minecraft mods sorted by version",
  metaDescription:
    "A Minecraft Java launcher for Windows. Sign in with Microsoft, install Modrinth mods in one click, and every version keeps its own mods.",
  skip: "Skip to content",
  nav: { features: "Features", how: "How it works", faq: "Questions", download: "Download", menu: "Open menu", close: "Close menu", other: "Español", otherLabel: "Leer en español" },

  heroEyebrow: "Minecraft Java · Fabric · Windows",
  heroTitle: ["Your mods, in their version.", "Without moving a folder."],
  heroText:
    "Sign in with your Microsoft account, pick a version and install Modrinth mods in one click. Every version keeps its own mods, so switching between them breaks nothing.",
  cta: "Download for Windows",
  ctaSource: "View the source",
  proof: "Free and open source · MIT or Apache 2.0 · Built in Rust",
  shotPlay: "The Play screen of Panda Launcher, with the panda and the sign in button",

  featuresEyebrow: "Features",
  featuresTitle: ["Made for players who run", "many versions with mods."],
  features: {
    folders: {
      title: "One folder per version",
      text: "1.21.10 and 26.3 never share mods. The launcher starts each version with its own folder, so there are no files to move when you switch.",
    },
    modrinth: {
      title: "Modrinth inside the launcher",
      text: "Search, read the description and pick the exact build. Dependencies install themselves.",
    },
    legal: {
      title: "Legal, with your account",
      text: "You sign in with Microsoft and the launcher checks that you own the game. There is no cracked mode.",
    },
    secrets: {
      title: "Your session stays on your machine",
      text: "Tokens live in Windows Credential Manager, never in files or logs.",
    },
    updates: {
      title: "Updates itself, signed",
      text: "Every new version is checked against an ed25519 signature and its SHA 256 before it installs.",
    },
  },
  treeCaption: "How your mods look on disk",

  tagline: ["Pick the version.", "Install what you want.", "Press play."],

  howEyebrow: "How it works",
  howTitle: ["From download to game", "in three steps."],
  steps: [
    { title: "Sign in", text: "Open the Microsoft link, type the code the launcher shows you and you are in. It checks that you own Minecraft.", alt: "Play screen with the button to sign in with Microsoft" },
    { title: "Find mods", text: "The Mods tab shows what is popular on Modrinth for your version, with icons and download counts.", alt: "Mods screen with cards for Fabric API, Sodium, Iris Shaders and other mods" },
    { title: "Pick a build and play", text: "Every mod has its own page with the description, links and every Fabric build for your version.", alt: "Sodium's page inside the launcher with stats, links and tabs" },
  ],

  faqEyebrow: "Questions",
  faqTitle: "What people usually ask",
  faq: [
    { q: "Do I need to buy Minecraft?", a: "Yes. Panda Launcher uses your Microsoft account and checks that you own Minecraft Java Edition before you play. There are no accounts without a purchase." },
    { q: "Is this an official Minecraft product?", a: "No. It is not approved by or associated with Mojang or Microsoft. It is an open source project." },
    { q: "Where are my mods stored?", a: "Each version has its own folder, for example profiles/1.21.10-fabric/mods, inside the launcher's data. You can open it and see them as normal files." },
    { q: "Does it work with Forge or NeoForge?", a: "No. The launcher uses Fabric only, and mods come from Modrinth only. That keeps every install predictable." },
    { q: "Can I play without internet?", a: "Yes, with an account that was already verified online on this computer. The first time needs a connection." },
    { q: "Installer or portable?", a: "The MSI installs for your user only, without admin rights, and shows up in the Start menu. The portable build is a single .exe file. Both update themselves." },
    { q: "What data does the launcher send?", a: "Only what it needs to work: Microsoft sign in, downloads from Mojang, Fabric and Modrinth, and the version check on GitHub. There is no telemetry or analytics." },
    { q: "Is there a macOS or Linux version?", a: "For now we only publish Windows builds. The code is cross platform Rust, so it may come later." },
  ],

  finalTitle: ["Install your mods once.", "Play on any version."],
  finalText: "Free, open source, and no extra accounts.",
  version: (v: string, date: string) => `Version ${v} · ${date}`,
  portable: (mb: string) => `or get the portable build (${mb} MB)`,
  msiSize: (mb: string) => `MSI installer · ${mb} MB`,
  noRelease: "No version has been published yet. You can follow the progress on GitHub.",
  releases: "All versions",

  footer: {
    disclaimer: "Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft.",
    privacy: "Privacy",
    license: "License",
    source: "Source code",
  },

  privacy: {
    title: "Privacy",
    description: "What data Panda Launcher uses and who it talks to.",
    updated: "Updated on 17 November 2026",
    sections: [
      { h: "The launcher", p: ["Panda Launcher has no servers of its own and collects no telemetry. It only talks to these services, and only for what it needs:", "Microsoft and Mojang to sign in, check that you own the game and download it. Fabric for the loader. Modrinth to search and install mods. GitHub to look for updates.", "Your email and personal data are never sent in any request. Requests identify themselves only with the launcher's name and its repository address."] },
      { h: "Your account", p: ["Your session tokens are stored in Windows Credential Manager. They are never written to files or logs. Signing out deletes them."] },
      { h: "This website", p: ["This site uses no cookies, analytics or third party scripts. It is hosted on GitHub Pages, which may log technical visit data under its own policy."] },
      { h: "Contact", p: ["For any question, open an issue on the GitHub repository."] },
    ],
    back: "Back to home",
  },
  notFound: { title: "This page does not exist", text: "The link may be broken, or the page has moved.", back: "Back to home" },
};

export const strings = { es, en } as const;
export const t = (lang: Lang) => strings[lang];
/** A path under the site base, e.g. `href("es/privacy/")`. */
export const href = (path = "") => `${import.meta.env.BASE_URL.replace(/\/$/, "")}/${path}`;
