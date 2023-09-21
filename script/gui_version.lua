Root = io.popen("git rev-parse --show-toplevel"):read("*l")
File = Root .. "/crates/gui/src-tauri/tauri.conf.json"
Context = io.open(File, "r"):read("*a")
LastVersion = string.match(Context, '"version": "([%d+.]*%d+)"')
io.write("Last version: " .. LastVersion .. "\n" .. "NewVersion: ")
NewVersion = io.read()

--- 更新 tauri.conf.json 文件中的版本号
Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

--- 更新 Cargo.toml 文件中的版本号
File = Root .. "/crates/gui/src-tauri/Cargo.toml"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'version = "([%d+.]*%d+)"', 'version = "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

--- 更新 App.tsx 文件中的版本号
File = Root .. "/crates/gui/src/App.tsx"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, '<div class="version">([%d+.]*%d+)</div>',
    '<div class="version">' .. NewVersion .. '</div>', 1)
io.open(File, "w"):write(Context)

--- 更新 package.json 文件中的版本号
File = Root .. "/crates/gui/package.json"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)
