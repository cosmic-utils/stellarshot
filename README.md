<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/ahoneybun/Stellarshot/main/res/icons/hicolor/scalable/apps/com.github.ahoneybun.Stellarshot.svg" width="150" />
  <h1>Stellarshot</h1>

  <h3>A simple backup application using Rustic for the COSMIC™ desktop.</h3>

  ![COSMIC Backups Light](https://raw.githubusercontent.com/ahoneybun/cosmic-backups/main/res/screenshots/COSMIC-Backups-Light.png#gh-light-mode-only)

  ![COSMIC Backups Dark](https://raw.githubusercontent.com/ahoneybun/cosmic-backups/main/res/screenshots/COSMIC-Backups-Dark.png#gh-dark-mode-only)
</div>

![main branch parameter](https://github.com/ahoneybun/cosmic-backups/actions/workflows/build.yml/badge.svg?branch=main)

# UNDER ACTIVE DEVELOPMENT

DO NOT TRUST this with important data at this time until it's first release. Always have multiple backups and follow the [3-2-1 rule](https://www.seagate.com/blog/what-is-a-3-2-1-backup-strategy/).

## This is not an official COSMIC™ application from System76

## Current features

- [x] Creating a repository with a hardcoded password
- [x] Creating a new snapshot (blank for now) into a selected repository

## Planned features

- [ ] Creating a repository with a user set password
- [ ] Create and delete snapshots into any selected repository

## Install

To install your COSMIC™ application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
just build-release
sudo just install
```
