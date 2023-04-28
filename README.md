# whereami
An extremely simple utility tool to get quick info on the host system. It is build on top of "sys_info" crate. It can support Linux, Mac OS X and Windows.

## Usage
```
$ whereami [options]
```

## Options
| Option              | Description                         |
| ------------------- | ----------------------------------- |
| -d, --disk          | get hard-disk information           |
| -r, --release-notes | get the OS release notes            |
| -m, --memory        | get ram and swap memory information |
| -h, --help          | display this help                   |

## Available information in Memory info
| Name       | Description                  |
| ---------- | ---------------------------- |
| Total      | The total size of RAM        |
| Used       | The total RAM in use         |
| Available  | The total available memory   |
| Free       | The total free memory        |
| Total swap | The total swap memory        |
| Free swap  | The total unused swap memory |
| Buffer     | The total buffer memory      |
| Cache      | The total cache memory       |

## Available information in Disk info
| Name       | Description                |
| ---------- | -------------------------- |
| Total      | The total size of the disk |
| Free Total | Available disk space       |

