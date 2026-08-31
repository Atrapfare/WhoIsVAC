@echo off
setlocal

rem Full local check: format, lint, test, release build.
rem The first failing step aborts the run.
rem Use "cargo fmt --check" below if the script should only report bad
rem formatting instead of fixing it.

cd /d "%~dp0"

echo === cargo fmt ===
cargo fmt
if errorlevel 1 goto :failed

echo.
echo === cargo clippy ===
cargo clippy --all-targets -- -D warnings
if errorlevel 1 goto :failed

echo.
echo === cargo test ===
cargo test
if errorlevel 1 goto :failed

echo.
echo === cargo build --release ===
cargo build --release
if errorlevel 1 goto :failed

echo.
echo All checks passed.
exit /b 0

:failed
echo.
echo FAILED - see the output above.
rem Keep the window open so the error stays readable after a double-click.
rem Pass any argument (check.cmd nopause) to skip this, e.g. from a script.
if "%~1"=="" pause
exit /b 1
