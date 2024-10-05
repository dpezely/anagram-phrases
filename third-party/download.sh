#! /bin/sh
set -e

# Custom word lists from http://wordlist.aspell.net are used for consistent
# test runs rather than assuming any particular language or localization that
# may be already installed on the host running tests.

if [ "$(which wget)" ]; then
    GET="wget -N --content-disposition"
elif [ "$(which curl)" ]; then
    GET="curl -R -J"
else
    echo "To download word lists, please install either wget or curl."
    exit 1
fi

set -x

# Dictionaries for word lists are from http://wordlist.aspell.net/.
# Create and download custimized Word Lists or Hunspell dictionaries from SCOWL
# http://app.aspell.net/create as of 2024-10-05
EN_AU='http://app.aspell.net/create?max_size=60&spelling=AU&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=tar.gz'
EN_CA='http://app.aspell.net/create?max_size=60&spelling=CA&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=tar.gz'
EN_US='http://app.aspell.net/create?max_size=60&spelling=US&max_variant=0&diacritic=strip&download=wordlist&encoding=utf-8&format=tar.gz'

[ -d en_AU ] || mkdir en_AU
[ -d en_CA ] || mkdir en_CA
[ -d en_US ] || mkdir en_US

(cd en_AU/ && $GET $EN_AU)
tar zxf en_AU/SCOWL-wl.tar.gz -C en_AU

(cd en_CA/ && $GET $EN_CA)
tar zxf en_CA/SCOWL-wl.tar.gz -C en_CA

(cd en_US/ && $GET $EN_US)
tar zxf en_US/SCOWL-wl.tar.gz -C en_US

echo 'Assets for test suite have been downloaded.'
echo 'From top-level directory for this repo, run: cargo test'
