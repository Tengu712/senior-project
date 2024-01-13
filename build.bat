@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

rem cl ^
rem   /c ^
rem   /Fo:vulkan-wrapper.obj ^
rem   ^
rem   /O2 /Oi ^
rem   /EHsc /Gd /GL /Gy ^
rem   /DNDEBUG /D_CONSOLE /D_UNICODE /DUNICODE ^
rem   /permissive- /Zc:inline ^
rem   /MD ^
rem   /FC /nologo /utf-8 ^
rem   /sdl /W4 /WX ^
rem   ^
rem   /I"C:\VulkanSDK\1.3.268.0\Include" ^
rem   "%tmpdir%\src\c\*.c"
rem 
rem lib ^
rem   /OUT:vulkan-wrapper.lib ^
rem   /LIBPATH:"C:\VulkanSDK\1.3.268.0\Lib" ^
rem   vulkan-1.lib ^
rem   vulkan-wrapper.obj

rustc ^
  --edition=2021 ^
  -L. ^
  -L "%tmpdir%\external\rand\target\release\deps" ^
  --extern rand="%tmpdir%\external\rand\target\release\librand.rlib" ^
  "%tmpdir%\src\rust\main.rs"

cd %tmpdir%
