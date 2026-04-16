/**
 * CLI Binary Manager
 *
 * Manages the lx binary:
 * - Detects platform and architecture
 * - Checks for existing installation in PATH
 * - Downloads pre-built binaries from GitHub releases
 * - Executes CLI commands
 */

import { spawn, spawnSync, execSync, type SpawnOptions } from 'node:child_process';
import { createWriteStream, existsSync, mkdirSync, chmodSync, unlinkSync, readFileSync } from 'node:fs';
import { platform, arch, homedir } from 'node:os';
import { join, dirname } from 'node:path';
import { pipeline } from 'node:stream/promises';
import { createGunzip } from 'node:zlib';
import { fileURLToPath } from 'node:url';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export interface CliConfig {
  /** GitHub repository (e.g., "owner/repo") */
  repo?: string;
  /** Specific version to use (e.g., "v0.1.0"), defaults to "latest" */
  version?: string;
  /** Custom binary path override */
  binaryPath?: string;
}

export interface ExecResult {
  stdout: string;
  stderr: string;
  exitCode: number;
}

interface CliConfigFile {
  repo: string;
  version?: string;
}

export interface ManualInstallHelp {
  command: string;
  repoUrl?: string;
  releasesUrl?: string;
}

interface GitHubAsset {
  name: string;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  html_url?: string;
  draft?: boolean;
  prerelease?: boolean;
  assets: GitHubAsset[];
}

export interface CompatibleLxRelease {
  tag: string;
  version: string;
  assetUrl: string;
  releaseUrl?: string;
}

export interface LxUpdateCheckResult {
  updateAvailable: boolean;
  currentVersion: string;
  latestVersion: string;
  releaseTag: string;
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const CLI_NAME = 'lx';
const CONFIG_FILE = 'cli-config.json';
const INSTALL_DIR = join(homedir(), '.lexiang', 'bin');

const PLATFORM_MAP: Record<string, string> = {
  darwin: 'apple-darwin',
  linux: 'unknown-linux-gnu',
  win32: 'pc-windows-msvc',
};

const ARCH_MAP: Record<string, string> = {
  arm64: 'aarch64',
  x64: 'x86_64',
};

// ---------------------------------------------------------------------------
// Config Loading
// ---------------------------------------------------------------------------

let cachedConfig: CliConfigFile | null = null;

function loadCliConfig(): CliConfigFile {
  if (cachedConfig) return cachedConfig;

  const __dirname = dirname(fileURLToPath(import.meta.url));
  const configPath = join(__dirname, '..', CONFIG_FILE);

  try {
    const content = readFileSync(configPath, 'utf-8');
    cachedConfig = JSON.parse(content) as CliConfigFile;
    return cachedConfig;
  } catch {
    return { repo: '' };
  }
}

export function getManualInstallHelp(config: CliConfig = {}): ManualInstallHelp {
  const cliConfig = loadCliConfig();
  const repo = config.repo ?? cliConfig.repo;

  return {
    command: 'cargo install lexiang-cli',
    repoUrl: repo ? `https://github.com/${repo}` : undefined,
    releasesUrl: repo ? `https://github.com/${repo}/releases` : undefined,
  };
}

// ---------------------------------------------------------------------------
// Platform Detection
// ---------------------------------------------------------------------------

function getTarget(): { target: string; ext: string } {
  const os = PLATFORM_MAP[platform()];
  const cpu = ARCH_MAP[arch()];

  if (!os || !cpu) {
    throw new Error(`Unsupported platform: ${platform()}-${arch()}`);
  }

  return {
    target: `${cpu}-${os}`,
    ext: platform() === 'win32' ? '.exe' : '',
  };
}

function getAssetName(): string {
  const { target } = getTarget();
  return `lx-${target}.tar.gz`;
}

// ---------------------------------------------------------------------------
// Binary Resolution
// ---------------------------------------------------------------------------

function findInPath(): string | null {
  try {
    const cmd = platform() === 'win32' ? 'where lx' : 'which lx';
    const result = execSync(cmd, { encoding: 'utf-8', stdio: ['pipe', 'pipe', 'ignore'] });
    const path = result.trim().split('\n')[0];
    return path && existsSync(path) ? path : null;
  } catch {
    return null;
  }
}

function getBundledPath(): string | null {
  const __dirname = dirname(fileURLToPath(import.meta.url));
  const { target, ext } = getTarget();
  const binaryPath = join(__dirname, '..', 'bin', `lx-${target}${ext}`);
  return existsSync(binaryPath) ? binaryPath : null;
}

function getInstalledPath(): string | null {
  const ext = platform() === 'win32' ? '.exe' : '';
  const binaryPath = join(INSTALL_DIR, `${CLI_NAME}${ext}`);
  return existsSync(binaryPath) ? binaryPath : null;
}

// ---------------------------------------------------------------------------
// Download
// ---------------------------------------------------------------------------

function normalizeLxVersion(raw: string): string {
  return raw.trim().replace(/^lexiang-cli\s+/i, '').replace(/^cli-/i, '').replace(/^v/i, '');
}

function compareIdentifiers(a: string, b: string): number {
  const numericPattern = /^\d+$/;
  const aIsNumeric = numericPattern.test(a);
  const bIsNumeric = numericPattern.test(b);

  if (aIsNumeric && bIsNumeric) {
    return Number(a) - Number(b);
  }

  if (aIsNumeric) return -1;
  if (bIsNumeric) return 1;

  const prereleaseOrder: Record<string, number> = {
    alpha: 1,
    beta: 2,
    gamma: 3,
    delta: 4,
    rc: 5,
  };

  const aRank = prereleaseOrder[a] ?? Number.MAX_SAFE_INTEGER;
  const bRank = prereleaseOrder[b] ?? Number.MAX_SAFE_INTEGER;
  if (aRank !== bRank) return aRank - bRank;

  return a.localeCompare(b);
}

export function compareLxVersions(a: string, b: string): number {
  const normalizedA = normalizeLxVersion(a);
  const normalizedB = normalizeLxVersion(b);

  const [coreA, prereleaseA] = normalizedA.split('-', 2);
  const [coreB, prereleaseB] = normalizedB.split('-', 2);

  const partsA = coreA.split('.').map((part) => Number(part) || 0);
  const partsB = coreB.split('.').map((part) => Number(part) || 0);
  const maxLength = Math.max(partsA.length, partsB.length);

  for (let i = 0; i < maxLength; i += 1) {
    const diff = (partsA[i] ?? 0) - (partsB[i] ?? 0);
    if (diff !== 0) return diff;
  }

  if (!prereleaseA && !prereleaseB) return 0;
  if (!prereleaseA) return 1;
  if (!prereleaseB) return -1;

  const identifiersA = prereleaseA.split('.');
  const identifiersB = prereleaseB.split('.');
  const prereleaseLength = Math.max(identifiersA.length, identifiersB.length);

  for (let i = 0; i < prereleaseLength; i += 1) {
    const left = identifiersA[i];
    const right = identifiersB[i];
    if (left === right) continue;
    if (left == null) return -1;
    if (right == null) return 1;

    const diff = compareIdentifiers(left, right);
    if (diff !== 0) return diff;
  }

  return 0;
}

function getCompatibleAsset(release: GitHubRelease): GitHubAsset | undefined {
  const assetName = getAssetName();
  return release.assets.find((asset) => asset.name === assetName);
}

function toCompatibleRelease(release: GitHubRelease): CompatibleLxRelease | null {
  const asset = getCompatibleAsset(release);
  if (!asset) return null;

  return {
    tag: release.tag_name,
    version: normalizeLxVersion(release.tag_name),
    assetUrl: asset.browser_download_url,
    releaseUrl: release.html_url,
  };
}

async function fetchGitHubRelease<T>(url: string): Promise<T> {
  const response = await fetch(url, {
    headers: { Accept: 'application/vnd.github.v3+json' },
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch release: ${response.status} ${response.statusText}`);
  }

  return (await response.json()) as T;
}

export async function getLatestCompatibleLxRelease(config: CliConfig = {}): Promise<CompatibleLxRelease> {
  const cliConfig = loadCliConfig();
  const repo = config.repo ?? cliConfig.repo;

  if (!repo) {
    throw new Error('No GitHub repository configured.');
  }

  const releases = await fetchGitHubRelease<GitHubRelease[]>(
    `https://api.github.com/repos/${repo}/releases?per_page=30`,
  );

  for (const release of releases) {
    if (release.draft || release.prerelease) continue;

    const compatible = toCompatibleRelease(release);
    if (compatible) return compatible;
  }

  throw new Error(`No compatible lx binary found for ${platform()}-${arch()} in recent releases.`);
}

async function getRelease(repo: string, version: string): Promise<CompatibleLxRelease> {
  if (version === 'latest') {
    return getLatestCompatibleLxRelease({ repo });
  }

  const release = await fetchGitHubRelease<GitHubRelease>(
    `https://api.github.com/repos/${repo}/releases/tags/${version}`,
  );
  const compatible = toCompatibleRelease(release);

  if (!compatible) {
    const assetName = getAssetName();
    throw new Error(
      `No binary for ${platform()}-${arch()} in release ${release.tag_name}.\n` +
        `Expected: ${assetName}\n` +
        `Available: ${release.assets.map((asset) => asset.name).join(', ') || 'none'}`,
    );
  }

  return compatible;
}

export async function checkForLxUpdate(
  currentVersion: string,
  config: CliConfig = {},
): Promise<LxUpdateCheckResult> {
  const cliConfig = loadCliConfig();
  const repo = config.repo ?? cliConfig.repo;
  const version = config.version ?? cliConfig.version ?? 'latest';
  const targetRelease = await getRelease(repo, version);
  const normalizedCurrent = normalizeLxVersion(currentVersion);

  return {
    updateAvailable: compareLxVersions(normalizedCurrent, targetRelease.version) < 0,
    currentVersion: normalizedCurrent,
    latestVersion: targetRelease.version,
    releaseTag: targetRelease.tag,
  };
}

async function downloadAndExtract(url: string, destDir: string): Promise<string> {
  mkdirSync(destDir, { recursive: true });

  const response = await fetch(url);
  if (!response.ok || !response.body) {
    throw new Error(`Download failed: ${response.status} ${response.statusText}`);
  }

  const ext = platform() === 'win32' ? '.exe' : '';
  const binaryPath = join(destDir, `${CLI_NAME}${ext}`);
  const tempTarGz = join(destDir, 'temp.tar.gz');
  const tempTar = join(destDir, 'temp.tar');

  // Download .tar.gz
  const fileStream = createWriteStream(tempTarGz);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  await pipeline(response.body as any, fileStream);

  // Gunzip
  const { createReadStream } = await import('node:fs');
  await pipeline(createReadStream(tempTarGz), createGunzip(), createWriteStream(tempTar));

  // Extract binary from tar
  const tar = readFileSync(tempTar);
  let offset = 0;
  while (offset < tar.length) {
    const header = tar.subarray(offset, offset + 512);
    if (header[0] === 0) break;

    const fileName = header.subarray(0, 100).toString('utf-8').replace(/\0/g, '');
    const sizeOctal = header.subarray(124, 136).toString('utf-8').replace(/\0/g, '').trim();
    const size = parseInt(sizeOctal, 8) || 0;

    if (fileName === CLI_NAME || fileName === `${CLI_NAME}${ext}` || fileName.endsWith(`/${CLI_NAME}`)) {
      const content = tar.subarray(offset + 512, offset + 512 + size);
      const { writeFileSync } = await import('node:fs');
      writeFileSync(binaryPath, content);
      chmodSync(binaryPath, 0o755);
    }

    offset += 512 + Math.ceil(size / 512) * 512;
  }

  // Cleanup
  try {
    unlinkSync(tempTarGz);
    unlinkSync(tempTar);
  } catch {
    // Ignore
  }

  if (!existsSync(binaryPath)) {
    throw new Error('Failed to extract binary from archive');
  }

  return binaryPath;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Download lx binary from GitHub releases.
 */
export async function downloadLxBinary(
  options: { version?: string; repo?: string } = {},
): Promise<string> {
  const cliConfig = loadCliConfig();
  const repo = options.repo ?? cliConfig.repo;

  if (!repo) {
    throw new Error(
      'No GitHub repository configured.\n' +
        'Please install lx manually: cargo install lexiang-cli',
    );
  }

  const version = options.version ?? cliConfig.version ?? 'latest';
  const { tag, assetUrl } = await getRelease(repo, version);

  console.log(`Downloading lx ${tag} from ${repo}...`);
  const binaryPath = await downloadAndExtract(assetUrl, INSTALL_DIR);

  return binaryPath;
}

/**
 * Get the path to the lx binary, downloading if necessary.
 */
export function getLxBinarySync(config: CliConfig = {}): string {
  if (config.binaryPath && existsSync(config.binaryPath)) {
    return config.binaryPath;
  }

  const pathBinary = findInPath();
  if (pathBinary) return pathBinary;

  const bundled = getBundledPath();
  if (bundled) return bundled;

  const installed = getInstalledPath();
  if (installed) return installed;

  throw new Error(
    'lx binary is not available for synchronous plugin registration.\n' +
      'Please install lx first or set binaryPath in plugin config.',
  );
}

export async function getLxBinary(config: CliConfig = {}): Promise<string> {
  // 1. Custom path
  if (config.binaryPath && existsSync(config.binaryPath)) {
    return config.binaryPath;
  }

  // 2. PATH
  const pathBinary = findInPath();
  if (pathBinary) return pathBinary;

  // 3. Bundled
  const bundled = getBundledPath();
  if (bundled) return bundled;

  // 4. Installed
  const installed = getInstalledPath();
  if (installed) return installed;

  // 5. Download
  const cliConfig = loadCliConfig();
  const repo = config.repo ?? cliConfig.repo;

  if (!repo) {
    throw new Error(
      'No GitHub repository configured.\n' +
        'Please install lx manually: cargo install lexiang-cli\n' +
        'Or set binaryPath in plugin config.',
    );
  }

  console.log(`Downloading lx from ${repo}...`);

  const version = config.version ?? cliConfig.version ?? 'latest';
  const release = await getRelease(repo, version);

  console.log(`Installing ${release.tag}...`);
  const binaryPath = await downloadAndExtract(release.assetUrl, INSTALL_DIR);
  console.log(`Installed to ${binaryPath}`);

  return binaryPath;
}

/**
 * Execute a lx command.
 */
export function execLxSync(
  args: string[],
  options: { accessToken?: string; cwd?: string } = {},
): ExecResult {
  const binary = getLxBinarySync();
  const env = { ...process.env };
  if (options.accessToken) {
    env.LEXIANG_ACCESS_TOKEN = options.accessToken;
  }

  const result = spawnSync(binary, args, {
    env,
    cwd: options.cwd,
    stdio: ['pipe', 'pipe', 'pipe'],
    encoding: 'utf-8',
  });

  if (result.error) {
    throw result.error;
  }

  return {
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
    exitCode: result.status ?? 1,
  };
}

export async function execLx(
  args: string[],
  options: { accessToken?: string; cwd?: string } = {},
): Promise<ExecResult> {
  const binary = await getLxBinary();

  return new Promise((resolve, reject) => {
    const env = { ...process.env };
    if (options.accessToken) {
      env.LEXIANG_ACCESS_TOKEN = options.accessToken;
    }

    const spawnOpts: SpawnOptions = {
      env,
      cwd: options.cwd,
      stdio: ['pipe', 'pipe', 'pipe'],
    };

    const proc = spawn(binary, args, spawnOpts);

    let stdout = '';
    let stderr = '';

    proc.stdout?.on('data', (data: Buffer) => {
      stdout += data.toString();
    });
    proc.stderr?.on('data', (data: Buffer) => {
      stderr += data.toString();
    });

    proc.on('error', reject);
    proc.on('close', (exitCode) => {
      resolve({ stdout, stderr, exitCode: exitCode ?? 1 });
    });
  });
}

/**
 * Execute a lx command and parse JSON output.
 */
export async function execLxJson<T = unknown>(
  args: string[],
  options: { accessToken?: string; cwd?: string } = {},
): Promise<T> {
  const jsonArgs = args.includes('--format') ? args : [...args, '--format', 'json'];
  const result = await execLx(jsonArgs, options);

  if (result.exitCode !== 0) {
    throw new Error(`lx failed (exit ${result.exitCode}): ${result.stderr || result.stdout}`);
  }

  try {
    return JSON.parse(result.stdout) as T;
  } catch {
    throw new Error(`Failed to parse JSON: ${result.stdout}`);
  }
}

/**
 * Check if lx is available without downloading.
 */
export function isLxAvailable(): boolean {
  return !!(findInPath() || getBundledPath() || getInstalledPath());
}
