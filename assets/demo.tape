Output demo.gif

Require dra

Set Width 900
Set Padding 20
Set Theme "Snazzy"
Set FontFamily "FiraCode Nerd Font"
Set FontSize 18

Type "dra"
Enter
Sleep 3s
Ctrl+L
Sleep 300ms

Hide
Type "# Interactive download"
Show
Sleep 2s
Ctrl+U
Type "dra download devmatteini/dra-tests"
Enter
Sleep 1s
Down@300ms 3
Sleep 300ms
Enter
Sleep 2s

Ctrl+L
Sleep 300ms

Hide
Type "# Non-Interactive download with selection mode"
Show
Sleep 2s
Ctrl+U
Type "dra untag devmatteini/dra-tests"
Enter
Sleep 1s
Down@150ms 11
Sleep 300ms
Enter
Sleep 2s

Type 'dra download -s "helloworld_{tag}.tar.gz" devmatteini/dra-tests'
Enter
Sleep 3s

Ctrl+L
Sleep 300ms

Hide
Type "# Automatic download based on your operating system"
Show
Sleep 2s
Ctrl+U
Type "dra download -a devmatteini/dra-tests"
Enter
Sleep 5s

Ctrl+L
Sleep 300ms

Hide
Type "# Install assets"
Show
Sleep 2s
Ctrl+U
Type 'dra download -a --install devmatteini/dra-tests'
Enter
Sleep 2s
Type "./helloworld"
Enter
Sleep 2s
