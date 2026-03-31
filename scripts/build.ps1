# Get project root
$RootDir = Split-Path -Path $PSScriptRoot -Parent
Set-Location $RootDir

Write-Host "`n[IRIS] Starting Full Environment Check & Build..." -ForegroundColor Cyan

# 1. Check for Virtual Environment
if (!(Test-Path ".venv")) {
    Write-Host "[*] .venv not found. Creating virtual environment..." -ForegroundColor Yellow
    python -m venv .venv
}

# 2. Activate Virtual Environment
Write-Host "[*] Activating environment..." -ForegroundColor Gray
& ".\.venv\Scripts\Activate.ps1"

# 3. Check/Install Maturin inside the venv
if (!(Get-Command maturin -ErrorAction SilentlyContinue)) {
    Write-Host "[*] Maturin not found. Installing..." -ForegroundColor Yellow
    pip install maturin
}

# 4. Build the Rust Engine
Write-Host "[*] Building iris_engine (Release)..." -ForegroundColor Magenta
maturin develop --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n[SUCCESS] iris_engine is ready in .venv!`n" -ForegroundColor Green
} else {
    Write-Host "`n[ERROR] Build failed." -ForegroundColor Red
}