# zipSniper

Read the directory of a ZIP Archive remotely. 

* Supports ZIP64
* Supports proxies (including Tor)
```
python .\zipsniper.py --help
usage: zipSniper [-h] [-c int] [--http_proxy url] [--https_proxy url] [-O OUTPUT_FILE] url

Pulls the directory structure of a zip file remotely

positional arguments:
  url                   Remote path of the zip archive

options:
  -h, --help            show this help message and exit
  -c int, --comment_buffer int
                        The size of the comment in EOCD is unknown. So to ensure we can find the EOCDs position in the ZIP archive, we specify a random buffer (Default: 1024 Bytes)
  --http_proxy url
  --https_proxy url
  -O OUTPUT_FILE, --output_file OUTPUT_FILE
```
