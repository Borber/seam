let Root = ( git rev-parse --show-toplevel )
let lastVersion = ($Root ++ '/crates/gui/src-tauri/tauri.conf.json' | open | get package.version)
let prompt = "Last version:" ++ $lastVersion ++ "\nNew version:"
let newVersion = (input $prompt | str trim)
let file = $Root ++ '/crates/gui/src/App.tsx'
$file | open | str replace ('<div class="version">' ++ $lastVersion ++ '</div>') ('<div class="version">' ++ $newVersion ++ '</div>') | save -f $file
let file = $Root ++ '/crates/gui/src-tauri/Cargo.toml';
$file | open | update package.version $newVersion | save -f $file
let file = $Root ++ '/crates/gui/src-tauri/tauri.conf.json'
$file | open | update package.version $newVersion | save -f $file
let file = $Root ++ '/crates/gui/package.json'
$file | open | update version $newVersion | save -f $file