# zipSniper

Read the directory of a ZIP Archive remotely. 

* Supports ZIP64
* Support for HTTP and SOCKS proxies in development
```
Extracts a file list within a zip archive remotely

Usage: zipSniper.exe [OPTIONS] --path <url>

Options:
  -p, --path <url>
  -c, --comment-buffer <BYTES>  Number of bytes to pull from the end of the file.
                                The EOCD checksum needs to land in this data chunk.
                                ZIP:0x06054b50  ZIP64:0x06054b50
                                 [default: 56]
  -o, --output-file <FILE>      Sets an optional output file
  -l, --log-level <LEVEL>       Sets the log level (error, warn, info, debug, trace) [default: info]
      --proxy <PROXY_URL>       Sets the proxy to route HTTP requests through
  -h, --help                    Print help
  -V, --version                 Print version
```
