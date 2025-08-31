# Image Optimizer RS - Windows Installation Script
# Requires PowerShell 5.1 or later

param(
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$RepoUrl = "https://github.com/nixuuu/image-optimizer"
$BinaryName = "image-optimizer.exe"

function Write-Header {
    Write-Host "================================================================" -ForegroundColor Cyan
    Write-Host "  Image Optimizer - Installation Script" -ForegroundColor Cyan
    Write-Host "  CLI tool for optimizing JPEG, PNG, and WebP images" -ForegroundColor Cyan
    Write-Host "================================================================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "❌ Error: $Message" -ForegroundColor Red
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor Yellow
}

function Test-Dependencies {
    Write-Info "Checking dependencies..."
    
    # Check if we can download files
    try {
        $null = Invoke-WebRequest -Uri "https://api.github.com" -Method Head -TimeoutSec 5
    }
    catch {
        Write-Error "Unable to connect to GitHub API. Check your internet connection."
        exit 1
    }
    
    # Check for Rust/Cargo
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Info "Rust/Cargo detected - will build from source"
        return "source"
    }
    else {
        Write-Info "Rust/Cargo not found - will try to download prebuilt binary"
        return "binary"
    }
}

function Get-Platform {
    $arch = $env:PROCESSOR_ARCHITECTURE
    
    switch ($arch) {
        "AMD64" { 
            $script:Architecture = "x86_64"
            $script:Target = "x86_64-pc-windows-msvc"
        }
        "ARM64" { 
            $script:Architecture = "aarch64"
            $script:Target = "aarch64-pc-windows-msvc"
        }
        default {
            Write-Error "Unsupported architecture: $arch"
            Write-Host "This installer supports x86_64 and aarch64 architectures only."
            exit 1
        }
    }
    
    Write-Info "Detected platform: Windows-$Architecture"
}

function Install-FromBinary {
    Write-Info "Attempting to download prebuilt binary..."
    
    $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    
    try {
        $releaseUrl = "https://api.github.com/repos/nixuuu/image-optimizer/releases/latest"
        
        Write-Info "Fetching latest release information..."
        $releaseData = Invoke-RestMethod -Uri $releaseUrl
        
        $asset = $releaseData.assets | Where-Object { $_.name -like "*$Target*" } | Select-Object -First 1
        
        if (-not $asset) {
            Write-Error "No prebuilt binary found for $Target (Windows-$Architecture)"
            Write-Info "Available binaries:"
            $releaseData.assets | ForEach-Object { Write-Host "  - $($_.name)" }
            return $false
        }
        
        $downloadUrl = $asset.browser_download_url
        $binaryPath = Join-Path $tempDir $asset.name
        
        Write-Info "Downloading $downloadUrl..."
        Invoke-WebRequest -Uri $downloadUrl -OutFile $binaryPath
        
        if (-not (Test-Path $binaryPath)) {
            Write-Error "Binary not found after download"
            return $false
        }
        
        # Create install directory
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }
        
        # Copy and rename binary
        $targetPath = Join-Path $InstallDir $BinaryName
        Copy-Item $binaryPath $targetPath -Force
        
        Write-Success "Binary installed to $targetPath"
        return $true
    }
    catch {
        Write-Error "Failed to download binary: $($_.Exception.Message)"
        return $false
    }
    finally {
        if (Test-Path $tempDir) {
            Remove-Item $tempDir -Recurse -Force
        }
    }
}

function Install-FromSource {
    Write-Info "Building from source..."
    
    $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    
    try {
        Write-Info "Cloning repository..."
        $repoDir = Join-Path $tempDir "image-optimizer"
        git clone $RepoUrl $repoDir
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to clone repository"
            exit 1
        }
        
        Push-Location $repoDir
        
        Write-Info "Building release binary..."
        cargo build --release
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Failed to build from source"
            exit 1
        }
        
        # Create install directory
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }
        
        # Copy binary
        $sourceBinary = Join-Path "target" "release" $BinaryName
        $targetPath = Join-Path $InstallDir $BinaryName
        Copy-Item $sourceBinary $targetPath -Force
        
        Write-Success "Built and installed to $targetPath"
    }
    finally {
        Pop-Location
        if (Test-Path $tempDir) {
            Remove-Item $tempDir -Recurse -Force
        }
    }
}

function Update-Path {
    $pathToAdd = $InstallDir
    
    # Check if already in PATH
    $currentPath = $env:PATH
    if ($currentPath -split ';' -contains $pathToAdd) {
        Write-Info "PATH already contains $pathToAdd"
        return
    }
    
    Write-Info "Adding $pathToAdd to user PATH..."
    
    try {
        # Get current user PATH from registry
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        
        if ($userPath) {
            $newPath = "$userPath;$pathToAdd"
        } else {
            $newPath = $pathToAdd
        }
        
        # Update registry
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        
        # Update current session
        $env:PATH = "$env:PATH;$pathToAdd"
        
        Write-Success "PATH updated successfully"
        Write-Info "The change will take effect in new PowerShell sessions"
    }
    catch {
        Write-Error "Failed to update PATH: $($_.Exception.Message)"
        Write-Info "You can manually add $pathToAdd to your PATH environment variable"
    }
}

function Test-Installation {
    $binaryPath = Join-Path $InstallDir $BinaryName
    
    if (Test-Path $binaryPath) {
        Write-Success "Installation verified"
        Write-Host ""
        Write-Host "🎉 Image Optimizer has been successfully installed!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Usage:" -ForegroundColor Cyan
        Write-Host "  $BinaryName --help                    # Show help"
        Write-Host "  $BinaryName -i .\images -r            # Optimize images recursively"
        Write-Host "  $BinaryName -i input -o output --quality 90  # Optimize with custom quality"
        Write-Host ""
        Write-Host "To use the tool immediately, either:" -ForegroundColor Cyan
        Write-Host "  1. Restart your PowerShell session, or"
        Write-Host "  2. Run: `$env:PATH = `"`$env:PATH;$InstallDir`""
        Write-Host "  3. Use the full path: $binaryPath"
        Write-Host ""
    }
    else {
        Write-Error "Installation verification failed"
        exit 1
    }
}

function Main {
    Write-Header
    Get-Platform
    $installMethod = Test-Dependencies
    
    Write-Host ""
    Write-Info "Installing Image Optimizer..."
    Write-Info "Install method: $installMethod"
    
    if ($installMethod -eq "binary") {
        if (-not (Install-FromBinary)) {
            Write-Info "Binary installation failed, falling back to source build..."
            
            if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
                Write-Error "git is required for source installation but not found"
                Write-Host "Please install Git from https://git-scm.com/download/win and try again"
                exit 1
            }
            
            if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
                Write-Error "Rust/Cargo is required for source installation but not found"
                Write-Host "Please install Rust from https://rustup.rs/ and try again"
                exit 1
            }
            
            Install-FromSource
        }
    }
    else {
        if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
            Write-Error "git is required but not installed"
            Write-Host "Please install Git from https://git-scm.com/download/win and try again"
            exit 1
        }
        
        Install-FromSource
    }
    
    Update-Path
    Test-Installation
}

# Only run main if script is executed directly (not sourced)
if ($MyInvocation.InvocationName -ne '.') {
    Main
}