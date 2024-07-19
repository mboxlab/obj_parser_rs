@echo off
set file_name=obj_parser_rs.dll

set target=x86_64-pc-windows-msvc
set target_dir=%cd%\target\%target%\release
set out=%cd%\gmcl_obj_parser_rs_win64.dll

rustup target add %target%
cargo build --release --target=%target%

move %target_dir%\%file_name% %out%
pause