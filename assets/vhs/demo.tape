Output docs/static/images/demo.gif
Output target/vhsout/

Set FontSize 20
Set Width    1500
Set Height   1000

Type  "# This is a demo of spellout, a command-line tool for encoding text into phonetic code."
Enter
Sleep 100ms
Type  "# The followings are demonstrations of how to use spellout to encode text into phonetic code."
Sleep 500ms
Enter
Type "# This is a simple example of spellout"
Sleep 200ms
Enter
Type "spellout Hello World"
Sleep 500ms
Enter
Sleep 1s

Type "# If no arguments are provided, spellout will read from standard input."
Sleep 200ms
Enter
Type "# Also, --only-code option will print only the encoded text without the original letters."
Sleep 200ms
Enter
Type "echo Hello World | spellout --only-code"
Sleep 500ms
Enter
Sleep 1s

Type "# By default, spellout will encode text into the NATO phonetic alphabet."
Sleep 200ms
Enter
Type "# You can specify a different code by the -c or --code option."
Sleep 200ms
Enter
Type "spellout --code uk Hello World"
Sleep 500ms
Enter
Sleep 1s

Type "# The available phonetic codes are listed in --list option."
Sleep 200ms
Enter
Type "spellout --list"
Sleep 500ms
Enter
Sleep 1s

Type "# Whole list of encoding rules of phonetic code is available in --print option."
Sleep 200ms
Enter
Type "spellout --code eu --print"
Sleep 500ms
Enter
Sleep 1s

Type "# spellout can uses your own custom phonetic code by --input option."
Sleep 200ms
Enter
Type "spellout --input testdata/custom_code.txt Hello World"
Sleep 500ms
Enter
Sleep 1s

Type "# spellout can also decode the given phonetic code back to the original text by --decode option."
Sleep 200ms
Enter
Type "spellout Hello World --only-code | spellout --decode"
Sleep 500ms
Enter
Sleep 1s

Type "# For more details, --help option, or please refer to the documentation."
Sleep 200ms
Enter
Type "# Thank you for watching this demo of spellout!"
Sleep 500ms
Enter
Type "spellout --help"
Enter
Sleep 5s
