// The newest GitHub release, read once at build time so visitors never hit the API rate limit.
// The release workflow redeploys the site, so the version shown is always the latest one.

const REPO = "juanestban/panda-launcher";
export const REPO_URL = `https://github.com/${REPO}`;

export type Asset = { url: string; mb: string };
export type Release = { version: string; date: string; page: string; msi?: Asset; portable?: Asset };

type GhAsset = { name: string; size: number; browser_download_url: string };

export async function latestRelease(): Promise<Release | null> {
  // Only the repo URL identifies us; no personal data in requests (CLAUDE.md, privacy rule).
  const headers: Record<string, string> = {
    Accept: "application/vnd.github+json",
    "User-Agent": `panda-launcher-web (+${REPO_URL})`,
  };
  if (process.env.GITHUB_TOKEN) headers.Authorization = `Bearer ${process.env.GITHUB_TOKEN}`;
  const response = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`, { headers });
  // 404: nothing published yet. The page then links to the releases list instead.
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`GitHub releases: HTTP ${response.status}`);
  const r = await response.json();
  const asset = (suffix: string): Asset | undefined => {
    const a = (r.assets as GhAsset[]).find((a) => a.name.endsWith(suffix));
    return a && { url: a.browser_download_url, mb: (a.size / 1048576).toFixed(1) };
  };
  return {
    version: String(r.tag_name).replace(/^v/, ""),
    date: String(r.published_at).slice(0, 10),
    page: r.html_url,
    msi: asset("-x64.msi"),
    portable: asset("-x64-portable.exe"),
  };
}
