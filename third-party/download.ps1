# PowerShell

# Custom word lists from http://wordlist.aspell.net are used for consistent
# test runs rather than assuming any particular language or localization that
# may be already installed on the host running tests.

New-Item -Path . -Name "en_AU" -ItemType "directory" -ErrorAction SilentlyContinue
New-Item -Path . -Name "en_CA" -ItemType "directory" -ErrorAction SilentlyContinue
New-Item -Path . -Name "en_US" -ItemType "directory" -ErrorAction SilentlyContinue

Invoke-RestMethod -Uri 'http://app.aspell.net/create?max_size=60&spelling=AU&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=zip' -Method Get -OutFile en_AU/SCOWL-wl.zip

Expand-Archive en_AU/SCOWL-wl.zip -DestinationPath en_AU

Invoke-RestMethod -Uri 'http://app.aspell.net/create?max_size=60&spelling=CA&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=zip' -Method Get -OutFile en_CA/SCOWL-wl.zip

Expand-Archive en_CA/SCOWL-wl.zip -DestinationPath en_CA

Invoke-RestMethod -Uri 'http://app.aspell.net/create?max_size=60&spelling=US&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=zip' -Method Get -OutFile en_US/SCOWL-wl.zip

Expand-Archive en_US/SCOWL-wl.zip -DestinationPath en_US

Write-Host 'Assets for test suite have been downloaded.'
Write-Host 'From top-level folder for this repo, run: cargo test'
