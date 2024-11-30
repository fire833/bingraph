Output demo.gif

Set Padding 15
Set FontSize 16
Set Width 1024
Set Height 756

Type "# First, you can compile the binary"
Sleep 500ms
Enter
Type "cargo build --release"
Enter
Sleep 3s

Type "# Now, run, optionally with a custom path for binaries and libraries"
Enter
Sleep 1.5s
Type "./target/release/bingraph -h"
Enter
Sleep 5s
Type "./target/release/bingraph --output-graphviz mypc.dot"
Sleep 5s
Enter
Sleep 15s
Type "# Your artifacts will be where you specified them"
Enter
Type "cat graph.json | tail -n 15"
Enter
Sleep 5s
Type "cat mypc.dot | head"
Enter
Sleep 5s
