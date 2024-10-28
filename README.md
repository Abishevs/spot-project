# S.P.O.T (Software/Study, Pomodoro, Organizer & Tracker)

![Status](https://img.shields.io/badge/Status-Under_Development-red)
![License](https://img.shields.io/badge/License-MIT-blue)
![Platform](https://img.shields.io/badge/Platform-Linux-yellowgreen)
![Rust](https://img.shields.io/badge/Language-Rust-orange)

## 📜 Overview

S.P.O.T project is monorepo. Essentialy a CLI tool designed to help you manage
and track your productivity with an built in Pomodoro. It allows you to set
custom Pomodoro sessions, track your progress, and store data locally or in a
database for further analysis.

## 📦 Features

- **Session Tracking** — Log time spent on specific subjects, tasks, or projects.
- **Statistics Over Time** — See how much time you've invested in each task (like "hours played" stats on Steam).
- **Pomodoro Timer** — Optional built-in Pomodoro feature for structured focus sessions.
- **Sound Notifications** — Plays different sounds for session start/end and break periods.
- **Persistent Data** — Tracks Pomodoro sessions, time in a task and stores them for review.
- **Service Setup** — Comes with a systemd service template for (`spot-daeom`).


## 🛠️ Installation

### Linux 
```bash
./scripts/install.sh
```

## Project structure
```sh
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── PKGBUILD
├── README.md
├── scripts
│   ├── cp-sounds.sh
│   ├── install.sh
│   ├── redeploy.sh
│   └── setup.sh
├── sounds                  
│   ├── break_end.mp3
│   ├── break_start.mp3
│   └── pomodoro_end.mp3
├── spot-cli
│   ├── Cargo.toml
│   ├── spot-cli-completions.zsh
│   └── src
│       ├── cli.rs
│       ├── fetcher.rs
│       ├── handler.rs
│       ├── main.rs
│       └── picker.rs
├── spot-daemon
│   ├── Cargo.toml
│   └── src
│       ├── config.rs
│       ├── database.rs
│       ├── handler.rs
│       ├── main.rs
│       ├── models
│       │   └── pomodoro.rs
│       ├── models.rs
│       ├── notify.rs
│       ├── service
│       │   ├── api.rs
│       │   └── pomodoro.rs
│       ├── service.rs
│       └── utils.rs
├── spot-lib
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── templates
    └── spotdaemon.service
```

## 🔗 Project Roadmap

Check out the [TODO list](./TODO.md) for planned features, improvements, and known issues.

## 📄 Changelog
See the [Changelog](./CHANGELOG.md) for details on version history and updates.

## Contibuting
Contributions are welcome! 

## 📄 License

This project is licensed under the GNU Affero General Public License v3.0.  
See the [LICENSE](./LICENSE) file for details.
