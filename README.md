# Privalytics
Open source, privacy-friendly, blazing fast analytics

## Progress
The basic functionality (submit, retrieve and delete) is complete.
[Some features are missing](docs/2-todo.md).

Check [4-libraries.md](docs/4-libraries.md) to see libraries that wrap the Privalytics API

## Objectives
- To create a _deploy and forget_ platform
- To create a _simple yet secure_ API
- To create a _fast and simple_ authentication system

## Setup
### From sources
1. Clone the repository and checkout the ``new`` branch
2. (With the rust toolchain installed) Run ``cargo build --release``
3. Copy the executable to your server
4. Create the following directories and files:
    - ``config/Config.toml`` (example at [config/Config.example.toml](config/Config.example.toml))
    - ``config/Users.toml`` (example at [config/Users.example.toml](config/Users.example.toml))
    - ``data/``
5. (In case you want the default UI) Copy the ``gui/`` folder
