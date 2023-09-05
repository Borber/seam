Root = io.popen("git rev-parse --show-toplevel"):read("*l")
File = Root .. "/crates/gui/src-tauri/tauri.conf.json"
Context = io.open(File, "r"):read("*a")
LastVersion = string.match(Context, '"version": "([%d+.]*%d+)"')
io.write("Last version: " .. LastVersion .. "\nNew version: ")
NewVersion = io.read()

Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

File = Root .. "/crates/gui/src-tauri/Cargo.toml"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, 'version = "([%d+.]*%d+)"', 'version = "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)

File = Root .. "/crates/gui/src/App.tsx"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, '<div class="version">([%d+.]*%d+)</div>',
    '<div class="version">' .. NewVersion .. '</div>', 1)
io.open(File, "w"):write(Context)

File = Root .. "/crates/gui/package.json"
Context = io.open(File, "r"):read("*a")
Context = string.gsub(Context, '"version": "([%d+.]*%d+)"', '"version": "' .. NewVersion .. '"', 1)
io.open(File, "w"):write(Context)
