@echo off

set tmpdir=%cd%
if not exist %~dp0bin mkdir %~dp0bin
cd %~dp0bin

glslc -o .\shader.vert.spv ..\src\shader\shader.vert
glslc -o .\shader.org.frag.spv ..\src\shader\shader.frag

cd %tmpdir%
