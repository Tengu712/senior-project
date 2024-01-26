@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

glslc -o .\shader.vert.spv ..\src\shader\shader.vert
glslc -o .\shader.frag.spv ..\src\shader\shader.frag

set clwith=cl ^
  /c ^
  ^
  /O2 /Oi ^
  /EHsc /Gd /GL /Gy ^
  /DNDEBUG /D_CONSOLE /D_UNICODE /DUNICODE ^
  /permissive- /Zc:inline ^
  /MD ^
  /FC /nologo /utf-8 ^
  /sdl /W4 /WX ^
  ^
  /I%VulkanInclude%
  
%clwith% /Fo:buffer.obj "%tmpdir%\src\c\util\memory\buffer.c"
%clwith% /Fo:image.obj  "%tmpdir%\src\c\util\memory\image.c"
%clwith% /Fo:memory.obj "%tmpdir%\src\c\util\memory\memory.c"
%clwith% /Fo:shader.obj "%tmpdir%\src\c\util\shader.c"
%clwith% /Fo:sub.obj  "%tmpdir%\src\c\sub.c"
%clwith% /Fo:main.obj "%tmpdir%\src\c\main.c"

lib ^
  /OUT:vulkan-wrapper.lib ^
  /LIBPATH:%VulkanLib% ^
  vulkan-1.lib ^
  *.obj

del *.obj

rustc ^
  --edition=2021 ^
  -L. ^
  -L "%tmpdir%\external\rand\target\release\deps" ^
  --extern rand="%tmpdir%\external\rand\target\release\librand.rlib" ^
  "%tmpdir%\src\rust\main.rs"

cd %tmpdir%
