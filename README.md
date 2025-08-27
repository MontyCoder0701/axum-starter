# axum-starter

This is a simple starter repository for axum projects.  
Use it for hackathons or external projects as a boilerplate — clone and customize as needed.

## What's Included

### GitHub Actions

- **Release Please** – automated release management
- **Dependabot** – automatic dependency updates
- **Dependabot Validation** - automatic build check for dependabot PRs
- **CI** – basic continuous integration for build, lint checks (clippy)

### Database

- **MySQL**
- [**Diesel**](https://diesel.rs)

## Notice

If you're using Dependabot, ensure the following setting is enabled:

> **Settings → Actions → General → Allow GitHub Actions to create and approve pull requests**

## How to run local server

Add and place `.env` in root directory. Fill in appropriate values for each.

```md
DATABASE_URL=
```

Install Diesel

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

Run initial setup

```sh
diesel setup
```

Run local server.

```sh
cargo run
```

Server is running on <http://localhost:3200>.

## Migrations

Generate migration

```sh
diesel migration generate --diff-schema {migration_name}
```

Run migration

```sh
diesel migration run
```

Revert migration with

```sh
diesel migration revert
```

Redo migration with

```sh
diesel migration redo
```

After changing schema, apply to migration

```sh
diesel migration generate --diff-schema {migration_name}
```

## Troubleshooting

<https://github.com/diesel-rs/diesel/issues/1286>

Make sure to setup system variables for MySQL in Windows.

```md
Write appropriate version number in {version}

MYSQLCLIENT_LIB_DIR=C:\Program Files\MySQL\MySQL Server {version}\lib

MYSQLCLIENT_VERSION={version}
```
