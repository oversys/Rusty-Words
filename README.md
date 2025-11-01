# Rusty Words

A project I'm working on to make it easier to learn words in another language. It's meant to have support for desktop and mobile (only Android tested).

Has import/export/delete database functionality to allow sharing the DB between devices.

The UI needs work on both desktop (doesn't fit a desktop program) and mobile (text and elements are too big).

If interested in the design, `design/sql/schema.sql` contains the SQL schemas based on the class diagram in `design/class_diagram/rusty_words.png`.

## Building APK
The `build_apk.sh` script can be used as a reference on how to build the APK. It may not work on all machines when run since some tools have to be set up first (see Tauri docs for [configuring mobile targets](https://v2.tauri.app/start/prerequisites/#android) and [distributing](https://v2.tauri.app/distribute/#android)).

## Frameworks/tools/software
- Tauri v2
- Vue 3
- SQLite (rusqlite)

