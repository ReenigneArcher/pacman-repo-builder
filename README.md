# PacMan Repo Builder

[![GitHub stars](https://img.shields.io/github/stars/lizardbyte/pacman-repo-builder.svg?logo=github&style=for-the-badge)](https://github.com/LizardByte/pacman-repo-builder)
[![GitHub Releases](https://img.shields.io/github/downloads/lizardbyte/pacman-repo-builder/total.svg?style=for-the-badge&logo=github)](https://github.com/LizardByte/pacman-repo-builder/releases/latest)
[![GitHub Workflow Status (CI)](https://img.shields.io/github/actions/workflow/status/lizardbyte/pacman-repo-builder/ci.yml.svg?branch=master&label=CI%20build&logo=github&style=for-the-badge)](https://github.com/LizardByte/pacman-repo-builder/actions/workflows/ci.yml?query=branch%3Amaster)

Build a custom pacman repository from a collection of PKGBUILD directories.

## Runtime Dependencies

* pacman
* makepkg
* libalpm.so.13
* libgit2.so

## Usage

**⚠ WARNING:** This program is meant to be used within a docker container.

### Manifest file

Manifest file is always named `build-pacman-repo.yaml`. It contains instruction to build a pacman repository.

**Example Manifest File:**

```yaml
# build-pacman-repo.yaml
global-settings:
  repository: repo/repo.db.tar.gz
  container: container
  read-build-metadata: either
  record-failed-builds: failed-builds.yaml
  install-missing-dependencies: false
  clean-before-build: false
  clean-after-build: false
  force-rebuild: true
  pacman: pacman
  arch-filter: [x86_64]
  check: inherit
  packager: Bob <bob@example.com>
  allow-failure: true
  dereference-database-symlinks: true
members:
  - directory: foo
  - directory: bar
    read-build-metadata: pkgbuild
    clean-before-build: false
    force-rebuild: true
    allow-failure: false
  - directory: bar
    install-missing-dependencies: true
    clean-after-build: false
    check: enabled
    pacman: yay
  - directory: baz
    read-build-metadata: srcinfo
    install-missing-dependencies: false
    clean-before-build: true
    clean-after-build: false
    force-rebuild: true
    check: disabled
    pacman: yay
    allow-failure: false
```

**Field Explanations:**

_Top-Level:_

| Field             | Type   | Description                                                                                                                        |
|-------------------|--------|------------------------------------------------------------------------------------------------------------------------------------|
| `global-settings` | object | Includes global settings from which all members inherit from.<br>Some settings can be overwritten by member customized properties. |
| `member`          | list   | List all members.                                                                                                                  |

_`global-settings`'s own fields:_

| Field                            | Type                           | Required/Optional                      | Description                                                                                                                        |
|----------------------------------|--------------------------------|----------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
| `repository`                     | `string`                       | required                               | Path to repository file (typically ends with `.db.tar.gz`).<br>It will be passed to `repo-add` command after each build.           |
| `container`                      | `string`                       | optional, default = `.`                | Directory that contains all build directories (a.k.a. members).                                                                    |
| `record-failed-builds`           | `string`                       | optional                               | If specified, old failed builds shall be skipped, and new failed builds shall be added to the file.                                |
| `arch-filter`                    | <code>"any" \| string[]</code> | optional, default = `any`              | Specify all CPU architectures to build.<br>Either `any` or an array of strings (e.g. `[x86_64, i686]`).                            |
| `packager`                       | `string`                       | optional, default = `Unknown Packager` | Identity of person or entity that produces the packages (i.e. the one who run this program).                                       |
| `dereference-database-symlinks`  | `boolean`                      | optional, default = `false`            | If `true`, all `*.db` and `*.files` symlinks will be converted to real files.                                                      |

_`member`'s own fields:_

| Field       | Type     | Required/Optional | Description                                                                       |
|-------------|----------|-------------------|-----------------------------------------------------------------------------------|
| `directory` | `string` | required          | Path to build directory of each member (relative to `global-settings.container`). |

_Shared Fields:_ Fields that exist in both `global-settings` and `member`. If `global-settings` and `member` both contain a field, `member`'s field will be prioritized.

| Field                          | Type                                         | Default<br>(`global-settings`) | Description                                                                                                                                                                                                           |
|--------------------------------|----------------------------------------------|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `read-build-metadata`          | <code>either \| srcinfo \| pkgbuild</code>   | `either`                       | Where to read build metadata:<br>`srcinfo`: From `.SRCINFO` file.<br>`pkgbuild`: From output of `makepkg --printsrcinfo`.<br>`either`: From `.SRCINFO` file if it exists, otherwise execute `makepkg --printsrcinfo`. |
| `install-missing-dependencies` | `boolean`                                    | `false`                        | Install packages found in `depends` and `makedepends` before each build.                                                                                                                                              |
| `clean-before-build`           | `boolean`                                    | `false`                        | Clean `$srcdir` and `$pkgdir` before each build.                                                                                                                                                                      |
| `clean-after-build`            | `boolean`                                    | `false`                        | Clean up after each build.                                                                                                                                                                                            |
| `force-rebuild`                | `boolean`                                    | `false`                        | Force build even if target package already exists.                                                                                                                                                                    |
| `check`                        | <code>enabled \| disabled \| inherit</code>  | `inherit`                      | Whether to add `--check` or `--nocheck` to `makepkg` command.                                                                                                                                                         |
| `pacman`                       | `string`                                     | `pacman`                       | Package manager program to use.<br>The program must recognize `pacman`'s CLI arguments and options.                                                                                                                   |
| `allow-failure`                | `boolean`                                    | `false`                        | If `false`, exits immediately when a build fails.<br>If `true`, ignore build failure should one occurs.                                                                                                               |

### Generate manifest file

Listing every member in a manifest file can be a chore. So when there are no members with customized properties, you
can generate the manifest file to reflect the build directories instead:

```sh
build-pacman-repo print-config \
  --repository $repo_dir/$repo_name.db.tar.gz \
  --container build-directories \
  --require-pkgbuild \
  --require-srcinfo \
  --with-install-missing-dependencies true \
  > build-pacman-repo.yaml
```

_Note:_ Replace `$repo_dir` with path of your repository directory. This directory will contain all built packages.
_Note:_ Replace `$repo_name` with name of your repository file. This file will be fetched by `pacman` to check for
updates.

### Replace `/usr/bin/makepkg` with one that allows running as root

The normal `makepkg` script does not allow running as root. While it may make sense in a user's machine,
it inconveniences a Docker container.

```sh
build-pacman-repo patch-makepkg --replace
```

### Build a pacman repositories

```sh
build-pacman-repo build
```

_Note:_ Make sure that `build-pacman-repo.yaml` file exists in current working directory.

### Print help message

```sh
build-pacman-repo help
```

```sh
build-pacman-repo --help
```

```sh
build-pacman-repo help $command
```

```sh
build-pacman-repo $command --help
```

## Real-world applications

* https://github.com/LizardByte/pacman-repo

## Frequently Asked Questions

### What does this program do?

The main purpose is to build a pacman repository. When the `build` command is called, it will read all source infos
from either `.SRCINFO` or `PKGBUILD`, sort them by their dependency relationship, then build them one by one.

### Why does this need to be run inside a container?

In order for this program to function properly, it must make several changes to the host system, such as:
* Replace `/usr/bin/makepkg` with one that allows running as root, so that it may be used in a CI environment.
* Install every built package just in case it may be depended upon by another package.
