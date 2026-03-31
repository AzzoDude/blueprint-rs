@echo off
setlocal
cd /d "%~dp0.."

echo [IRIS] Checking Environment...
if not exist ".venv" (
    echo [*] Creating .venv...
    python -m venv .venv
)

echo [*] Activating .venv and Building...
call .venv\Scripts\activate.bat && (
    pip show maturin >nul 2>&1 || pip install maturin
    maturin develop --release
)

if %errorlevel% neq 0 (
    echo [ERROR] Build failed!
    pause
    exit /b %errorlevel%
)

echo [SUCCESS] Build Complete.
timeout /t 3