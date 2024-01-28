@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

rustc ^
  -o ga.exe ^
  --edition=2021 ^
  -L "%tmpdir%\external\rand\target\release\deps" ^
  --extern rand="%tmpdir%\external\rand\target\release\librand.rlib" ^
  "%tmpdir%\src\rust\main.rs"

cd %tmpdir%
