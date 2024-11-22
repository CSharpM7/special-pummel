function Get-TimeStamp {
    
    return "[{0:MM/dd/yy} {0:HH:mm:ss}]" -f (Get-Date)
    
}

$modPath = Get-Item -Path .\modPath.txt | Get-Content -Tail 1

If ($args[0] -like "*devhook*") {
    cargo skyline install --install-path $modPath --features="devhook"
    Write-Output "$(Get-TimeStamp) Installed devhook plugin"
}
elseif ($args[0] -like "*dev*") {
    cargo skyline install --install-path rom:/smashline/development.nro --features="dev"
    Write-Output "$(Get-TimeStamp) Installed dev plugin"
}
else{
    cargo skyline install --install-path $modPath
    Write-Output "$(Get-TimeStamp) Installed plugin"
}