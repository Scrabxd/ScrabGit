@echo off
if "%~1"=="" (
    echo Use: scrab [comando]
    echo Allowed commands:
    echo   init
    echo   commit
    exit /b
)

set command=%~1

rem 
goto check_command

:check_command
if "%command%"=="init" goto init
if "%command%"=="commit" goto commit

goto unknown

:init
echo Initializing...
REM 
goto end

:commit
echo makinga  commit...
REM 
goto end

:unknown
echo Command "%command%" not recognized.
echo Use: scrab [comando]
echo Allowes commands:
echo   init
echo   commit

:end
