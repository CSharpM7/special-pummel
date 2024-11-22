$cargoContents = Get-Content -Path cargo.toml
$pluginName = $cargoContents[1].replace('name = ','').replace('"','')

$pluginLIB = "target/aarch64-skyline-switch/release/lib"+$pluginName+".nro"
$pluginNRO = "target/aarch64-skyline-switch/release/plugin.nro"
if (Test-Path $pluginNRO) {
    Remove-Item $pluginNRO
}

cargo skyline build --release
Move-Item -Path $pluginLIB -Destination $pluginNRO

Invoke-Item "target/aarch64-skyline-switch/release"