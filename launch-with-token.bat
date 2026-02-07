@echo off
REM GitHub Token Launcher for Godot PR Launcher
REM 
REM Usage: Set your GitHub Personal Access Token below
REM or pass it as a parameter: launch-with-token.bat YOUR_TOKEN_HERE

REM === CONFIGURATION ===
REM Replace YOUR_GITHUB_TOKEN_HERE with your actual token
REM OR pass the token as the first parameter when running this file
SET GITHUB_TOKEN=%1

REM If no parameter provided, use the default token below
IF "%GITHUB_TOKEN%"=="" (
    SET GITHUB_TOKEN=YOUR_GITHUB_TOKEN_HERE
)

REM === LAUNCH APPLICATION ===
echo Starting Godot PR Launcher with GitHub authentication...
echo Token: %GITHUB_TOKEN:~0,8%... (hidden for security)
echo.

REM Launch the application with the token as environment variable
START "" godot-pr-launcher.exe

echo.
echo Application launched!
echo.
echo Note: Keep this window open or the token will not persist.
echo Press any key to close this launcher...
pause >nul
