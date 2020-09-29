# SSH Config Tool
## What is this
Tools for manipulating the following files.
+ .ssh/config
+ .ssh/known_hosts

## How to
This tool has the following four modes.
1. select mode
2. tool settings mode
3. configs file mode
4. known_hosts file mode
   
### selsct mode
Move to each mode.

|input||
|-|-|
|config| move to configs file mode|
|known_host| move to known_host file mode|
|setting| move to tool settings mode|
|exit|exit to this tool|

### tool settings mode
Make settings for this tool.
The configuration file is listed in setting.txt

|input||
|-|-|
|edit|Edit to setting file|
|save|Write all settings to setting.txt|
|show|Print all current settings|
|exit|exit to this mode|

### donfig file mode
Set the config_file_path file described in settings.txt as the .ssh/config file

|input||
|-|-|
|new|Create new config|
|edit|Edit to config|
|delete|Delete to config|
|show|Print all current config|
|out|Write to config file|
|path|Print config file path|
|exit|exit to this mode|

### known_hosts file mode
Set the known_host_file_path file described in settings.txt as the .ssh/known_hosts file

|input||
|-|-|
|delete|Delete to known_hosts|
|show|Print all current setting|
|out|Write to known_hosts file|
|path|Print known_hosts file path|
|exit|exit to this mode|
