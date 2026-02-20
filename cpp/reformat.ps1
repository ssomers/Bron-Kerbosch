Get-ChildItem -Recurse -Include *.cpp,*.h -Depth 1 |
ForEach-Object {
    Write-Host $_
    &"C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\Llvm\\x64\\bin\\clang-format.exe" -i --fail-on-incomplete-format $_
}
