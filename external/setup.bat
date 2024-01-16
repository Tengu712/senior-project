@echo off

set tmpdir=%cd%

if not exist ".\rand" (
  git clone https://github.com/rust-random/rand.git
  cd rand
  cargo build --release
)

if not exist ".\stb_image.h" (
  bitsadmin /transfer stb_image https://raw.githubusercontent.com/nothings/stb/master/stb_image.h %tmpdir%\stb_image.h
)

if not exist ".\stb_image_write.h" (
  bitsadmin /transfer stb_image_write https://raw.githubusercontent.com/nothings/stb/master/stb_image_write.h %tmpdir%\stb_image_write.h
)

cd %tmpdir%
