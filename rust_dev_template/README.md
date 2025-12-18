# rust dev template

## structure in dev/

### .devcontainer
```
.devcontainer/
    .env
    devcontainer.json
    docker-copmose.yml
    Dockerfile
```
`.env` file is copied from `../.env.example`
#### configuration
* docker-compose.yml
    * postgresql image version
* Dockerfile
    * rust image version

### cargo workspace
```
Cargo.toml
crates/
    app/
        Cargo.toml
        src/
    lib1/
        Cargo.toml
        src/
    lib2/
        Cargo.toml
        src/
    ...
```
Cargo.toml (at the top directory) includes:
```
[workspace]
members = [ "crates/*" ]
```
Each app/libs will be created by `cargo new app` (or `--lib lib1`)

### deploy
```
deploy/
    releases/
        current -> yyyy-mm-dd_hhmmss
        yyyy-mm-dd_hhmmss/
            .env.example
            docker-compose.yml
            app/myapp  # executable
            docker/app/Dockerfile  # runtime image configuration
    db/
        data/
        init/
            001_schema.sql
            002_seed.sql
    scripts/
        release.sh
        release-unpack.sh
        release-switch.sh
        run.sh.example
        db-backup.sh
        db-restore.sh
```
* releases/current symlink points to the execution version
* at dev env:
    * `scripts/release.sh [app_name]`
* at prod env:
    * `scripts/release-unpack.sh [app_name] [version]`
        * unpack releases/${app_name}-${version}.tar.gz to releases/${version}/
    * `release-switch.sh [target_version]`
* at prod env: run as scripts/run.sh.example
#### configuration
* releases/current/docker/app/Dockerfile
    * s/myapp/${APP_NAME}/g
    * `EXPOSE 8080`
* releases/current/docker-compose.yml
    * s/myapp/${APP_NAME}/g

### scripts
```
scripts/
    check.sh
    bootstrap-pack.sh
    bootstrap-unpack.sh.example
```
* `check.sh` invokes `cargo fmt`, `cargo clippy`, `cargo test`
* `bootstrap-pack.sh` create initial deploy assets
    * include: deploy/db/, deploy/scripts/

### other files
* .env.example
    * be copied into `.devcontainer/.env` (for dev use), and `deploy/releases/.env.example`
    * for production use, env vars are passed by command line.
* .gitignore
    * must includes ".env", ".devcontainer/.env"

