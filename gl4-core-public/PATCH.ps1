
# Запусти этот .ps1 в E:\Programming\rust\OfLLM\gl4-core-public\
Write-Host "Patching gl4-core-public/src/ ..."
Copy-Item -Force .\src\fixed.rs .\src\fixed.rs.bak -ErrorAction SilentlyContinue
Copy-Item -Force $PSScriptRoot\src\fixed.rs .\src\fixed.rs
Copy-Item -Force $PSScriptRoot\src\tables.rs .\src\tables.rs
Copy-Item -Force $PSScriptRoot\src\fnc_ai.rs .\src\fnc_ai.rs
Copy-Item -Force $PSScriptRoot\src\lib.rs .\src\lib.rs
Copy-Item -Force $PSScriptRoot\src\types.rs .\src\types.rs -ErrorAction SilentlyContinue
Copy-Item -Force $PSScriptRoot\benches\gl4_bench.rs .\benches\gl4_bench.rs
Write-Host "Patched. Running cargo check..."
cargo check -p gl4-core-public
if ($LASTEXITCODE -eq 0) { Write-Host "OK - теперь bench" -ForegroundColor Green; cargo bench -p gl4-core-public --bench gl4_bench -- --nocapture } else { Write-Host "Ошибки остались" -ForegroundColor Red }
