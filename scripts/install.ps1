# Vibe CLI Windows Installation Script

$repo = "anjia/vibe-cli"
$installDir = "$HOME\.local\bin"
$binaryName = "vibe.exe"

Write-Host "==> Installing Vibe CLI..." -ForegroundColor Blue

# Create install directory
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Get latest release tag
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"
try {
    $latestRelease = Invoke-RestMethod -Uri $apiUrl
    $latestTag = $latestRelease.tag_name
} catch {
    Write-Error "Could not find latest release for $repo."
    exit 1
}

$downloadUrl = "https://github.com/$repo/releases/download/$latestTag/vibe-windows-x64.zip"
$zipPath = "$env:TEMP\vibe.zip"

Write-Host "==> Downloading $latestTag..." -ForegroundColor Blue
Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath

Write-Host "==> Extracting..." -ForegroundColor Blue
Expand-Archive -Path $zipPath -DestinationPath $installDir -Force
Remove-Item $zipPath

Write-Host "==> Vibe CLI installed successfully to $installDir\$binaryName" -ForegroundColor Green

# Update PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-Host "==> Adding $installDir to User PATH..." -ForegroundColor Blue
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    $env:Path = "$userPath;$installDir"
}

Write-Host "==> Done! Try running 'vibe check' in a new terminal." -ForegroundColor Green
