@echo off
setlocal EnableDelayedExpansion
title Nux Version Manager

:: Colors
set "CYAN=[36m"
set "GREEN=[32m"
set "YELLOW=[33m"
set "WHITE=[37m"
set "NC=[0m"

cls
echo.
echo %CYAN%    ╔═══════════════════════════════════════════════════════════════════╗%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ║    ████     ██████████████      ███╗    ██╗██╗    ██╗██╗    ██╗   ║%NC%
echo %CYAN%    ║    ████     ██████████████      ████╗   ██║██║    ██║╚██╗  ██╔╝   ║%NC%
echo %CYAN%    ║    ████     ████                ██╔██╗  ██║██║    ██║ ╚██╗██╔╝    ║%NC%
echo %CYAN%    ║    ████     ████                ██║╚██╗ ██║██║    ██║  ╚███╔╝     ║%NC%
echo %CYAN%    ║    ██████████████████████       ██║ ╚██╗██║██║    ██║   ███║      ║%NC%
echo %CYAN%    ║    ██████████████████████       ██║  ╚████║██║    ██║  ██╔██╗     ║%NC%
echo %CYAN%    ║             ████     ████       ██║   ╚███║██║    ██║ ██╔╝╚██╗    ║%NC%
echo %CYAN%    ║             ████     ████       ██║    ╚██║██║    ██║██╔╝  ╚██╗   ║%NC%
echo %CYAN%    ║    █████████████     ████       ██║     ╚█║╚██████╔╝██║      ██║  ║%NC%
echo %CYAN%    ║    █████████████     ████       ╚═╝      ╚╝ ╚═════╝ ╚═╝      ╚═╝  ║%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ║               %WHITE%Programming Language%CYAN% v1.0.0                          ║%NC%
echo %CYAN%    ║                                                                   ║%NC%
echo %CYAN%    ╚═══════════════════════════════════════════════════════════════════╝%NC%
echo.
echo     %YELLOW%Version Management%NC%
echo.
echo     %GREEN%●%NC% %WHITE%1.0.0%NC% (Current, Installed)
echo.
echo     %CYAN%ℹ%NC% No other versions are currently available.
echo.
pause
