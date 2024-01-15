@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

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
  /I"C:\VulkanSDK\1.3.268.0\Include"
  
%clwith% /Fo:buffer.obj "%tmpdir%\src\c\util\memory\buffer.c"
%clwith% /Fo:image.obj  "%tmpdir%\src\c\util\memory\image.c"
%clwith% /Fo:memory.obj "%tmpdir%\src\c\util\memory\memory.c"
%clwith% /Fo:file.obj   "%tmpdir%\src\c\util\file.c"
%clwith% /Fo:shader.obj "%tmpdir%\src\c\util\shader.c"
%clwith% /Fo:sub.obj  "%tmpdir%\src\c\sub.c"
%clwith% /Fo:main.obj "%tmpdir%\src\c\main.c"

lib ^
  /OUT:vulkan-wrapper.lib ^
  /LIBPATH:"C:\VulkanSDK\1.3.268.0\Lib" ^
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
