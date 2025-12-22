@echo off
if "%1"=="" (
    echo Usage: build.bat ^<filename_without_extension^>
    echo Example: build.bat hello_world
    echo This will build hello_world.asm into hello_world.exe
    goto :end
)

set filename=%1

echo Assembling %filename%.asm...
nasm -f win64 -o %filename%.obj %filename%.asm
if errorlevel 1 (
    echo Assembly failed!
    goto :end
)

echo Linking %filename%.obj...
link out.obj "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\um\x64\kernel32.Lib" "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\ucrt\x64\ucrt.lib" "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.35.32215\lib\x64\vcruntime.lib" "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.35.32215\lib\x64\store\legacy_stdio_definitions.lib" "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.35.32215\lib\x64\store\legacy_stdio_wide_specifiers.lib" /subsystem:console /entry:mainCRTStartup /LARGEADDRESSAWARE:NO
if errorlevel 1 (
    echo Linking failed!
    goto :end
)

echo Build successful! Running %filename%.exe...
echo.
%filename%.exe
echo.
echo Exit code: %errorlevel%

:end
pause