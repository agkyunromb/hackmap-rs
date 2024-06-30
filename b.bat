@echo off
cd/d "%~dp0"
cls
cargo build --features "113c"
copy /y "target\i686-pc-windows-msvc\debug\D2Sigma.dll" D:\Game\median-xl\
copy /y "target\i686-pc-windows-msvc\debug\D2Sigma.pdb" D:\Game\median-xl\