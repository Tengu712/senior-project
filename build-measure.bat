@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

cl ^
  /Fe:measure.exe ^
  ^
  /O2 /Oi ^
  /EHsc /Gd /GL /Gy ^
  /DNDEBUG /D_CONSOLE /D_UNICODE /DUNICODE ^
  /permissive- /Zc:inline ^
  /MD ^
  /FC /nologo /utf-8 ^
  /sdl /W4 /WX ^
  ^
  /I%VulkanInclude% ^
  ..\src\c\util\memory\*.c ^
  ..\src\c\util\*.c ^
  ..\src\c\*.c ^
  ^
  /link ^
  /LIBPATH:%VulkanLib% ^
  vulkan-1.lib ^
  user32.lib

del *.obj

cd %tmpdir%
