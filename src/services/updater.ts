import pkg from '../../package.json';

export const APP_VERSION = pkg.version;

export interface UpdateCheckResult {
  status: 'up-to-date' | 'update-available' | 'no-releases' | 'rate-limited' | 'error';
  currentVersion: string;
  latestVersion?: string;
  releaseName?: string;
  releaseUrl?: string;
  downloadUrl?: string;
  publishedAt?: string;
  releaseNotes?: string;
  checkedAt: string;
  message: string;
}

export function parseSemVer(v: string): number[] {
  const cleaned = v.trim().replace(/^v/i, '');
  const parts = cleaned.split('.').map((p) => {
    const num = parseInt(p, 10);
    return isNaN(num) ? 0 : num;
  });
  while (parts.length < 3) parts.push(0);
  return parts.slice(0, 3);
}

export function isNewerVersion(latest: string, current: string): boolean {
  const [lMajor, lMinor, lPatch] = parseSemVer(latest);
  const [cMajor, cMinor, cPatch] = parseSemVer(current);

  if (lMajor !== cMajor) return lMajor > cMajor;
  if (lMinor !== cMinor) return lMinor > cMinor;
  return lPatch > cPatch;
}

export async function checkForUpdates(currentVersion: string = APP_VERSION): Promise<UpdateCheckResult> {
  const checkedAtTime = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  const repo = 'nitinduseja/auto-dark-mode';
  const url = `https://api.github.com/repos/${repo}/releases/latest`;

  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 7000);

    const res = await fetch(url, {
      headers: {
        Accept: 'application/vnd.github.v3+json',
      },
      signal: controller.signal,
    });
    clearTimeout(timeoutId);

    if (res.status === 404) {
      return {
        status: 'no-releases',
        currentVersion,
        checkedAt: checkedAtTime,
        message: `No public releases found yet on GitHub. You are running the latest version (v${currentVersion}).`,
      };
    }

    if (res.status === 403) {
      return {
        status: 'rate-limited',
        currentVersion,
        checkedAt: checkedAtTime,
        message: 'GitHub API rate limit exceeded. Please try again in a few minutes.',
      };
    }

    if (!res.ok) {
      return {
        status: 'error',
        currentVersion,
        checkedAt: checkedAtTime,
        message: `GitHub API returned HTTP ${res.status}: ${res.statusText}`,
      };
    }

    const data = await res.json();
    const latestTag = (data.tag_name || '').trim();
    const hasNewer = isNewerVersion(latestTag, currentVersion);

    let downloadUrl = data.html_url;
    if (Array.isArray(data.assets)) {
      const exeAsset = data.assets.find((a: { name?: string; browser_download_url?: string }) =>
        a.name?.toLowerCase().endsWith('.exe')
      );
      if (exeAsset?.browser_download_url) {
        downloadUrl = exeAsset.browser_download_url;
      }
    }

    if (hasNewer) {
      return {
        status: 'update-available',
        currentVersion,
        latestVersion: latestTag,
        releaseName: data.name || latestTag,
        releaseUrl: data.html_url,
        downloadUrl,
        publishedAt: data.published_at,
        releaseNotes: data.body,
        checkedAt: checkedAtTime,
        message: `A newer version (${latestTag}) is available!`,
      };
    } else {
      return {
        status: 'up-to-date',
        currentVersion,
        latestVersion: latestTag,
        releaseUrl: data.html_url,
        checkedAt: checkedAtTime,
        message: `AutoDarkMode is up to date (v${currentVersion})`,
      };
    }
  } catch (err: unknown) {
    const isAbort = err instanceof DOMException && err.name === 'AbortError';
    return {
      status: 'error',
      currentVersion,
      checkedAt: checkedAtTime,
      message: isAbort
        ? 'Request timed out while checking for updates.'
        : 'Unable to reach GitHub. Please check your internet connection.',
    };
  }
}
