# Pithekos
The Xenizo build of a Pankosmia editor which also acts as a reference use case

## Setup and configuration
This repo pulls together several libraries and projects into a single Tauri app. The projects are spread across several repos to allow modular reuse. The sanest way to get this working is to install the repos with the following structure:

```
pankosmia
-- pithekos
-- core-client-dashboard repository
-- core-client-settings repository
-- core-client-workspace repository 
-- core-client-remote-repos repository
-- core-client-i18n-editor repository
```

*It Should Just Work* if your pankosmia directory is under `repos` under your user directory, ie `/home/myname/repos/pankosmia` in Linux.

The `setup` directory contains
- `app_setup.json` - this defines the list of clients to be used within Pithekos. Changing this file changes Pithekos for everyone. Keep the variable substitution so that the project works for everyone.
- `local_setup.json` - this file is auto-generated the first time you run Pithekos. Do not commit it! You may change the path to your pankosmia directory here if necessary.

For testing you may add clients to the `my_clients` array in `user_settings.json` in `pankosmia_working`, eg
```
"my_clients": [
    {"path": "/path/to/my/client/buildDir"}
  ],
```

Pankosmia-web serves compiled files from the `build` directory of each client, so you need to build each client:
```
# In each client repo, NOT this repo!
npm install
npm run build
```

## Required libraries
With Ubuntu 24.04 you may need
```
sudo apt install libglib2.0-dev
sudo apt install libpango1.0-dev
sudo apt install libwebkit2gtk-4.1-dev
sudo apt install libatk1.0-dev
```

#### Tested on Ubuntu 24.04 with:
- npm 9.2.0
- node 18.19.1
- rustc 1.83.0 -- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Tested on Windows 11 with:
- npm 10.7.0
- node 18.20.4
- rustc 1.83.0 -- See https://www.rust-lang.org/tools/install
- cmake 3.31.0 -- _Version 3 is required._ See https://cmake.org/download/

## Running in dev mode
```
npm install
npm run tauri dev
```

