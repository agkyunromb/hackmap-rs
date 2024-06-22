@echo off
cd/d "%~dp0"
cls
cargo build --release -Z build-std --target i686-win7-windows-msvc
copy /y "target\i686-win7-windows-msvc\release\D2Sigma.dll" .
