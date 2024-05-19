<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/ahoneybun/cosmic-backups/main/res/icons/hicolor/256x256/apps/com.example.CosmicAppTemplate.svg" width="150" />
  <h1>COSMIC Backups</h1>

  <h3>A simple backup application using Rustic for the COSMIC desktop.</h3>

  ![COSMIC Backups Light](https://raw.githubusercontent.com/ahoneybun/cosmic-backups/main/res/screenshots/COSMIC-Backups-Light.png#gh-light-mode-only)

  ![COSMIC Backups Dark](https://raw.githubusercontent.com/ahoneybun/cosmic-backups/main/res/screenshots/COSMIC-Backups-Dark.png#gh-dark-mode-only)
</div>

The current plan is to use Restic as the backend but for now the UI/UX is still being worked on. I have no idea if this will move forward past the init stage.

![main branch parameter](https://github.com/ahoneybun/cosmic-backups/actions/workflows/build.yml/badge.svg?branch=main)

## Install

To install your COSMIC application, you will need [just](https://github.com/casey/just), if you're on Pop!\_OS, you can install it with the following command:

```sh
sudo apt install just
```

After you install it, you can run the following commands to build and install your application:

```sh
just build-release
sudo just install
```
