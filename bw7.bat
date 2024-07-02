@echo off
cd/d "%~dp0"
cls
cargo build --features "113c" --release -Z build-std --target i686-win7-windows-msvc
copy /y "target\i686-win7-windows-msvc\release\D2Sigma.dll" D:\Game\median-xl\
