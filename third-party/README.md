# Third-Party Files

Custom word lists from http://wordlist.aspell.net are used for consistent
test runs rather than assuming any particular language or localization that
may be already installed on the host running tests.

Please download English language dictionaries for Australia (`EN_AU`),
Canada (`EN_CA`) and United State of America (`EN_US`), and use their form
http://app.aspell.net/create with the following parameters.

Parameters:

- max_size=60
- max_variant=0
- diacritic=strip
- download=wordlist
- encoding=utf-8
- format=tar.gz

Scripts with those options encoded into URLs are provided in this
subdirectory.
