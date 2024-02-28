# Version increment-er

Small tool that, when provided with a version
string, it return the version with the patch version incremented.

If [semantic version string](https://semver.org/) is provided, the tool incremets the patch number and discards any prerelease and build metadata.

If version string doesn't follow semantic versioning standard, the tool adds **.1** string to the end of the version string. If the version string already has **.x** postfix, the number **x** is incremented instead. See examples.

## Build

```sh
$ cargo build
```

## Usage
```sh
$ ./vincrementer version_string
```

## Examples

```sh
$ ./vincrementer 1.2.3
1.2.4
$ ./vincrementer 1.2.3-alpha.7
1.2.4
$ ./vincrementer version_string
version_string.1
$ ./vincrementer version_string.3
version_string.4
```


