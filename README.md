<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/ahoneybun/Stellarshot/main/res/icons/hicolor/scalable/apps/com.github.ahoneybun.Stellarshot.svg" width="150" />
  <h1>Stellarshot</h1>

  <h3>A simple backup application using Rustic for the COSMIC™ desktop.</h3>

  ![Stellarshot Light](https://raw.githubusercontent.com/cosmic-utils/stellarshot/main/res/screenshots/Stellarshot-Light.png#gh-light-mode-only)

  ![Stellarshot Dark](https://raw.githubusercontent.com/cosmic-utils/stellarshot/main/res/screenshots/Stellarshot-Dark.png#gh-dark-mode-only)
</div>

![main branch parameter](https://github.com/cosmic-utils/stellarshot/actions/workflows/build.yml/badge.svg?branch=main)

# UNDER ACTIVE DEVELOPMENT

This application should NOT be trusted at this moment, up until its first release, important data is at risk. Always have multiple backups and follow the [3-2-1 rule](https://www.seagate.com/blog/what-is-a-3-2-1-backup-strategy/).

## This is not an official COSMIC™ application from System76

## Current features

- [x] Create a repository with a user set password
- [x] Create a new snapshot in the selected repository with custom file(s)
- [x] Delete a selected repository
- [x] Delete snapshots in the selected repository

## Planned features

- [ ] Create a snapshot with selected folder(s) and/or file(s)
- [ ] Restore from a snapshot

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
