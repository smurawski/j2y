Simple CLI tool to convert JSON to YAML or YAML to JSON

```
PS > .\target\debug\j2y.exe --help
J 2 Y 0.0.221
Steven Murawski <steven.murawski@microsoft.com>
Converts JSON between YAML

USAGE:
    j2y-linux [FLAGS] [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Include verbose output

OPTIONS:
    -s, --source <SourceFormat>    Input serialization format. [default: JSON]  [values: YAML, JSON]

ARGS:
    <INPUT>     Sets the input file to use
    <OUTPUT>    Sets the output file to use
```