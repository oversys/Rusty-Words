# Rusty Words

A project I'm working on to make it easier to save learned words in another language for quick reference. Since the project is built using Tauri, it should support all platforms. However, only Android and Linux have been tested.

Has import/export/delete database functionality to allow sharing the DB between devices.

If interested in the design, `design/sql/schema.sql` contains the SQL schemas based on the class diagram in `design/class_diagram/rusty_words.png`.

## Building APK
The `build_apk.sh` script can be used as a reference on how to build the APK. It may not work on all machines when run since some tools have to be set up first (see Tauri docs for [configuring mobile targets](https://v2.tauri.app/start/prerequisites/#android) and [distributing on Android](https://v2.tauri.app/distribute/#android)).

## Building on Linux
Running `npm run tauri build` will build the program as Debian & RPM packages as well as an AppImage.

To install on Arch Linux, use `debtap` to convert the `.deb` package to an Arch Linux package, then install using `pacman -U [package]`.

See Tauri docs for [distributing on Linux](https://v2.tauri.app/distribute/#linux).

## Frameworks/tools/software
- Tauri v2
- Vue 3
- SQLite (rusqlite)

