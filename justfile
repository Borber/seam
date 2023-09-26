[private]
default:
    @just --list

# 编译 CLI
cd:
    cargo build --package seam -r

# 编译 GUI
gb:
    cd ./crates/gui; \
    yarn tauri build

# 调试 GUI
gd:
    cd ./crates/gui; \
    yarn tauri dev

# 更新 GUI 版本号
gv:
    @lua ./script/gui_version.lua

# 更新 GUI 依赖
gu:
    cd ./crates/gui; \
    yarn upgrade-interactive  --latest; \
    cargo update
