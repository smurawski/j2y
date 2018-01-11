param (
    [switch]$Format,
    [switch]$Build,
    [switch]$Clean,
    [switch]$Check,
    [switch]$Release,
    [switch]$Test,
    [switch]$TriggerLinux
)
function Test-RustUp {
    (get-command -Name rustup.exe -ErrorAction SilentlyContinue) -ne $null
}

function Test-RustFmt {
    (get-command -Name rustfmt.exe -ErrorAction SilentlyContinue) -ne $null
}


function Get-CargoCommand {
    param ([switch]$Nightly)
    $Branch = 'stable'
    if ($Nightly) {$Branch = 'nightly'}
    if (Test-RustUp) {
        "cargo +$branch-x86_64-pc-windows-msvc "
    }
    else {
        "$env:USERPROFILE/.cargo/bin/cargo.exe +$branch-x86_64-pc-windows-msvc "
    }
}

function Get-RustupCommand {
    if (Test-RustUp) {
        "rustup.exe "
    }
    else {
        "$env:USERPROFILE/.cargo/bin/rustup.exe "
    }
}

function Invoke-Build([string]$Path, [switch]$Clean, [switch]$Release, [switch]$Check) {
    $Path = Resolve-Path $Path

    $cargo = Get-CargoCommand

    Push-Location "$Path"
    if ($Clean) {
        Write-Host "$cargo clean"
        invoke-expression "$cargo clean" -ErrorAction Stop
    }
    if ($Check) {
        Write-Host "$cargo check"
        Invoke-Expression "$cargo check" -ErrorAction Stop
    }
    else {
        Write-Host "$cargo build $(if ($Release) { '--release' })"
        Invoke-Expression "$cargo build $(if ($Release) { '--release' })" -ErrorAction Stop
    }
    Pop-Location
}
function New-PathString([string]$StartingPath, [string]$Path) {
    if (-not [string]::IsNullOrEmpty($path)) {
        if (-not [string]::IsNullOrEmpty($StartingPath)) {
            [string[]]$PathCollection = "$path;$StartingPath" -split ';'
            $Path = ($PathCollection |
                    Select-Object -Unique |
                    Where-Object {-not [string]::IsNullOrEmpty($_.trim())} |
                    Where-Object {test-path "$_"}
            ) -join ';'
        }
        $path
    }
    else {
        $StartingPath
    }
}

function Assert-Rust {
    param ([switch]$Nightly)
    $Branch = 'stable'
    if ($Nightly) {$Branch = 'nightly'}
    Write-Host "Validating Rust ($branch-x86_64-pc-windows-msvc) is installed and up to date."
    if (-not (Test-RustUp)) {
        Write-Host "Installing rustup and $branch-x86_64-pc-windows-msvc Rust."
        invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
        ./rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc --no-modify-path
    }
    
    $rustup = Get-RustupCommand
    invoke-expression "$rustup install $branch-x86_64-pc-windows-msvc"
    
}


Write-Host "Making sure ~/.cargo/bin is on PATH"
$env:PATH = New-PathString -StartingPath "$env:PATH" -Path "$env:USERPROFILE\.cargo\bin"

if ($Format) {
    $cargo = Get-CargoCommand -nightly

    if (-not (Test-RustFmt)) {
        Assert-Rust -nightly        
        Write-Host "Installing rustfmt"
        Invoke-Expression "$cargo install rustfmt-nightly"
    }
    Write-Host "$cargo fmt -- --write-mode diff"
    invoke-expression "$cargo fmt -- --write-mode diff --verbose"
}

if ($Test) {
    Assert-Rust
    $cargo = Get-CargoCommand

    if ($Clean) {
        invoke-expression "$cargo clean"
    }
    Invoke-Expression "$cargo test $(if ($Release) { '--release' })" -ErrorAction Stop
}

if ($Build) {
    Assert-Rust
    Invoke-Build -path $pwd -Clean:$Clean -Release:$Release -Check:$Check
}

if ($TriggerLinux) {
    $url = "$($env:SYSTEM_TEAMFOUNDATIONCOLLECTIONURI)$env:SYSTEM_TEAMPROJECTID/_apis/build/builds?api-version=2.0"
    $body = "{ 'definition' : { 'id' : $env:LINUXBUILD_DEFINITIONID }, 'sourceBranch' : '$env:BUILD_SOURCEBRANCH' }"
    $type = "application/json"
    $headers = @{
        Authorization = "Bearer $env:SYSTEM_ACCESSTOKEN"
    }
    Write-Host "URL: $url"
    $definition = Invoke-RestMethod -Uri $url -Body $body -ContentType $type -Method Post -Headers $headers
    Write-Host "Triggered Linux Build"
}
