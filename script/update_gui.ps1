$file = "../crates/gui/src-tauri/tauri.conf.json";
$content = Get-Content $file | Out-String
$content -match '"version": "(\d+\.\d+\.\d+)"' | Out-Null
$lastVersion = $Matches[1]
$newVersion = Read-Host -Prompt "Last version: $lastVersion`nNew version"
$file = "../crates/gui/src/App.tsx";
$content = Get-Content $file
$newContent = $content -replace '<div class="version">\d+\.\d+\.\d+</div>', "<div class=`"version`">$newVersion</div>"
Set-Content $file $newContent
$file = "../crates/gui/src-tauri/Cargo.toml";
$content = Get-Content $file
$newContent = $content -replace '^version = "\d+\.\d+\.\d+"', "version = `"$newVersion`""
Set-Content $file $newContent
$file = "../crates/gui/src-tauri/tauri.conf.json";
$content = Get-Content $file
$newContent = $content -replace '"version": "\d+\.\d+\.\d+"', "`"version`": `"$newVersion`""
Set-Content $file $newContent
$file = "../crates/gui/package.json";
$content = Get-Content $file
$newContent = $content -replace '"version": "\d+\.\d+\.\d+"', "`"version`": `"$newVersion`""
Set-Content $file $newContent
