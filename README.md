RenameX (rnx)
=============


This cli tool is mainly intended to be used when facing
problems in transferring files from Google Drive (which does not
have any significant file name length limits) to other cloud
providers using rclone or similar software.

Usage
-----

After cloning the repo run the following command to
generate the binary in the target/release folder

```shell
cargo build --release
```

This tool takes two mandatory params

path - the directory where the files are

cap - the max file name length (extension included)

```shell
rnx --path . --cap 130
```

The above command will look at all files in the current
directory and shorten them to a 130 byte size.

```shell
rnx --path . --cap 130 --sub
```

Providing the --sub flag will also traverse subdirectories

```shell
rnx --path . --cap 130 --ascii
```

Providing the --ascii flag will replace all non-ascii characters with printable
ascii (they may still be non-ascii however. Check the function
calculate_name in find_and_rename)

```shell
rnx --path . --cap 130 --dry
```

Providing the --dry flag will only display the
names of the files and not make any changes