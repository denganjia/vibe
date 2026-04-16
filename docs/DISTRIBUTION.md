# Distribution Guide

This document outlines the steps to distribute `vibe` to various package managers.

## 1. GitHub Releases (The Source)

Currently, the primary distribution method is via GitHub Releases.
- **Automation**: The `.github/workflows/release.yml` triggers on any tag matching `v*`.
- **Artifacts**: It produces optimized binaries for macOS (x64/arm64), Linux (x64), and Windows (x64).

### How to release a new version:
1. Update version in `Cargo.toml`.
2. Commit and push.
3. Create a tag: `git tag v0.1.0 && git push origin v0.1.0`.
4. GitHub Actions will handle the rest.

---

## 2. Homebrew (macOS)

To support `brew install anjia/tap/vibe`, you need to create a "Tap" repository.

### Setup:
1. Create a GitHub repo named `homebrew-tap`.
2. Add a `vibe.rb` formula.

### Formula Template:
```ruby
class Vibe < Formula
  desc "Physical orchestration layer for AI agents"
  homepage "https://github.com/anjia/vibe-cli"
  url "https://github.com/anjia/vibe-cli/releases/download/v0.1.0/vibe-macos-arm64.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  version "0.1.0"

  def install
    bin.install "vibe"
  end
end
```

### Automation:
You can use `goreleaser/brew-tap` patterns in GitHub Actions to update this formula automatically.

---

## 3. Winget (Windows)

Winget requires submitting a manifest to the `microsoft/winget-pkgs` repository.

### Tools:
Use the [Yamtool](https://github.com/microsoft/winget-create) or manual PRs.

---

## 4. One-line Install (Unix/macOS)

Users can install via:
```bash
curl -sSL https://raw.githubusercontent.com/anjia/vibe-cli/master/scripts/install.sh | bash
```

*(Note: Replace `master` with your default branch and ensure the REPO variable in the script is correct.)*
