# rust dev template
* rust (axum + sqlx + tracing (json log) + dotenvy) + postgresql
* develop in vscode devcontainer

## structure in dev/

### .devcontainer
```
.devcontainer/
    devcontainer.json
    docker-copmose.yml
    Dockerfile
```
#### configuration point
* rust version
    - Dockerfile: `FROM rust:1.95-slim-trixie`
* postgresql version
    - docker-compose.yml: `services: db: image: postgres:18`
* service port
    - docker-compose.yml: `services: app: ports: - "8080:8080"`
* docker service name
    - to change service name 'app':
        - docker-compose.yml: `services: app:`
        - devcontainer.json: `"service": "app",`
    - to change service name 'db':
        - docker-compose.yml: `services: app: depends_on: db:`, `services: db:`
* docker volume name
    - to change volume name 'db-data':
        - docker-compose.yml: `services: db: volumes: - db-data:/var/lib/postgresql`, `volumes: db-data:`

### cargo workspace
```
Cargo.toml
.env.dev  # copy from '.env.example'
.env.test  # copy from '.env.example'
crates/
    myapp_app/
        Cargo.toml
        migrations/
            *.sql
        src/
            main.rs
            run.rs
            config/
                convert.rs
            ...
    myapp_db/
        Cargo.toml
        src/
            query.rs
            query/
                *.rs
            ...
    myapp_web/
        Cargo.toml
        src/
            lib.rs
            route.rs
            route/
                *.rs
            ...
deploy/db/init/001_schema.sql
```
* DB creation is done by 'deploy/db/init/001_schema.sql'
    - this file is also refered by docker-compose.yml (in .devcontainer/ and deploy/releases/)
* DDL (CREATE TABLE, etc.) and initial data are provided through migration by sqlx
    - migration files are 'myapp_app/migrations/*.sql'
#### configuration point
* .env.dev and .env.test
    - copy from '.env.example', then configure them
* app name
    - myapp_app/ directory
        - it's directory name
    - myapp_db/ directory
        - it's directory name
        - myapp_app/Cargo.toml: `myapp_db = { path = "../myapp_db" }`
        - myapp_app/src/main.rs: `use myapp_db::*;`
        - myapp_web/Cargo.toml: `myapp_db = { path = "../myapp_db" }`
        - myapp_web/src/lib.rs: `use myapp_db::*;`
    - myapp_web/ directory
        - it's directory name
        - myapp_app/Cargo.toml: `myapp_web = { path = "../myapp_web" }`
        - myapp_app/src/main.rs: `use myapp_web::*;`
* db name
    - deploy/db/init/001_schema.sql: `CREATE DATABASE app WITH ENCODING = 'UTF8';`
* web server listen address/port
    - myapp_web/src/lib.rs: `const WEB_SERVER_LISTEN_ADDR: &str = "0.0.0.0:8080";`
    - docker-compose.yml (in .devcontainer/ and deploy/releases/): `services: app: ports: - "8080:8080"`
* app main processing
    - myapp_app/src/run.rs: in function `pub async fn run(config: Config, db: db::Db) -> ExitCode {`
* env variables
    - each .env files
    - myapp_app/src/config.rs: in struct `pub struct Config {`
    - myapp_app/src/config.rs: in function `Config::from_env()` (`pub fn from_env() -> Result<Self, LoadError> {`)
* add new config conversion
    - myapp_app/src/config/convert.rs: add any `impl From<&Config> for YOUR_CONFIG {`
* db queries
    - refer to myapp_db/src/query/users.rs
    - myapp_db/src/query/*.rs
    - myapp_db/src/query.rs: `pub mod xxx;`
* migration sqls
    - myapp_app/migrations/*.sql: YYYYMMDDhhmmss_what_is_this_file_do.sql
* web service endpoint
    - refer to myapp_web/src/route/example.rs
    - myapp_web/src/route/*.rs
    - myapp_web/src/route.rs: `pub mod xxx;`, `.merge(xxx::router())` (in function `router()`)
* if you don't need web, remove them:
    - myapp_web/
    - myapp_app/Cargo.toml: `myapp_web = { path = "../myapp_web" }`
    - myapp_app/src/main.rs: `use myapp_web::*;`
    - myapp_app/src/config/convert.rs: `use crate::server::ServerConfig;`
    - myapp_app/src/config/convert.rs: `impl From<&Config> for ServerConfig { ... }`
    - unused env variables:
        - `WEB_SERVER_RPS`
        - `WEB_SERVER_TIMEOUT_SECS`
    - (edit main processing file: myapp_app/src/run.rs)
* if you don't need db, remove them:
    - myapp_db/
    - edit: myapp_web/src/state.rs
    - edit: myapp_web/src/server.rs
    - edit: myapp_web/route/*.rs
    - myapp_web/src/lib.rs: `use myapp_db::*;`
    - myapp_app/Cargo.toml: `myapp_db = { path = "../myapp_db" }`
    - myapp_app/src/config/convert.rs: `use crate::db::DbConfig;`
    - myapp_app/src/config/convert.rs: `impl From<&Config> for DbConfig { ... }`
    - if only command 'migrate' implemented: myapp_app/src/cmd.rs
    - edit: myapp_app/src/main.rs
    - (edit main processing file: myapp_app/src/run.rs)
    - myapp_app/migrations/
    - deploy/db/
    - deploy/scripts/db-backup.sh
    - deploy/scripts/db-restore.sh
    - deploy/scripts/migrate.sh

### deploy
```
deploy/
    releases/
        docker-compose.yml
        docker/app/Dockerfile
        app/
            myapp  # executable, copy by 'scripts/release-pack.sh'
        .env.example  # copy by 'scripts/release-pack.sh'
        [app_name]-YYYY-MM-DD_hhmmss.tar.gz  # create by 'scripts/release-pack.sh'
    db/
        data/
        init/
            001_schema.sql
    scripts/
        release-unpack.sh
        release-switch.sh
        migrate.sh
        db-backup.sh
        db-restore.sh
```
* 'db/' used by docker db image (docker-compose.yml in '.devcontainer/' and 'deploy/releases/')
* 'scripts/*.sh' described in below section 'structure image on PROD environment'
#### configuration point
* runtime os version
    - releases/docker/app/Dockerfile: `FROM debian:trixie-slim`
* postgresql version
    - releases/docker-compose.yml: `services: db: image: postgres:18`
* service port
    - releases/docker-compose.yml: `services: app: ports: - "8080:8080"`
* app name
    - releases/docker/app/Dockerfile:
        - `COPY ./app/myapp /app/myapp`
        - `RUN chmod +x /app/myapp`
        - `CMD ["/app/myapp"]`
* docker service name
    - to change service name 'app':
        - releases/docker-compose.yml: `services: app:`
    - to change service name 'db':
        - releases/docker-compose.yml: `services: app: depends_on: db:`, `services: db:`
* docker container name
    - to change container name 'myapp':
        - releases/docker-compose.yml:
            - `services: app: image: myapp:latest`
            - `services: app: container_name: myapp`
    - to change container name 'myapp_db':
        - releases/docker-compose.yml: `services: db: container_name: myapp_db`
* docker network name
    - to change network name 'myappnet':
        - releases/docker-compose.yml:
            - `services: app: networks: myappnet`
            - `services: db: networks: myappnet`
            - `networks: myappnet:`

### scripts
```
scripts/
    bootstrap-pack.sh
    bootstrap-unpack.sh
    check.sh
    release-pack.sh
    fix-permissions.sh
```
* will be used on DEV environment
    - otherwise 'deploy/scripts/*.sh' will be used on PROD environment
* scripts/bootstrap-pack.sh
    - create tar-ball includes initial deploy assets
    - include: 'deploy/db/', 'deploy/scripts/'
    - put them into 'scripts/../bootstrap.tar.gz'
* scripts/bootstrap-unpack.sh
    - will be used on PROD environment
    - extract from './bootstrap.tar.gz'
    - create directory './releases/'
* scripts/check.sh
    - invokes `cargo fmt`, `cargo clippy`, `APP_ENV=test cargo test`
* scripts/release-pack.sh
    - Usage: ./release-pack.sh [app_name]
    - build and copy 'target/release/[app_name]' to 'deploy/releases/app/.'
    - copy '.env.example' to 'deploy/releases/.'
    - create tar-ball include them:
        - deploy/releases/docker-compose.yml
        - deploy/releases/docker/
        - deploy/releases/app/
        - deploy/releases/.env.example
    - put them into 'deploy/releases/[app_name]-YYYY-MM-DD_hhmmss.tar.gz'
* scripts/fix-permissions.sh
    - change file permissions to 0664
    - created files in devcontainer become '0644 root:[host-user-group]'
    - so the files can not be edited on out-of-container environment
    - this script add w permission to group, then host-user-group members can edit it

### other files
* .env.example
    - be copied into '.env.dev' (for dev use), '.env.test' (for test use), and 'deploy/releases/.env.example'
    - for production use, .env file must be permission 0400
* .gitignore
    - must includes '.env*', '!.env.example', '.env*/'

## structure image on PROD environment
```
bootstrap.tar.gz  # deploy here at initial bootstrap
bootstrap-unpack.sh  # deploy here at initial bootstrap
releases/
    current -> YYYY-MM-DD_hhmmss  # switch link by 'scripts/release-switch.sh'
    [app_name]-YYYY-MM-DD_hhmmss.tar.gz  # deploy here
    YYYY-MM-DD_hhmmss/  # created by 'scripts/release-unpack.sh'
        docker-compose.yml
        docker/app/Dockerfile
        app/myapp  # executable
        .env.example
db/
    data/
    init/
        001_schema.sql
scripts/
    release-unpack.sh
    release-switch.sh
    migrate.sh
    db-backup.sh
    db-restore.sh
```
* 'releases/', 'db/', and 'scripts/' are extracted from 'bootstrap.tar.gz'
    - use 'bootstrap-unpack.sh'
### deploy
* deploy file to 'releases/[app_name]-YYYY-MM-DD_hhmmss.tar.gz'
* 'releases/YYYY-MM-DD_hhmmss/' extracted from 'releases/[app_name]-YYYY-MM-DD_hhmmss.tar.gz'
    - use 'scripts/release-unpack.sh [app_name] [YYYY-MM-DD_hhmmss]'
* configure 'releases/current/.env' from 'releases/current/.env.example'
    - .env file must be `chmod 0400 .env`
* switch current application by 'scripts/release-switch.sh [YYYY-MM-DD_hhmmss]'
* run migration by 'scripts/migrate.sh [app_name] [db_service_name]'
    - run 'docker compose up -d [db_service_name]'
    - run 'docker compose run --rm [app_name] migrate'
    - run 'docker compose up -d [app_name]'
    - this script has [--clear-db] option flag, but it is useless:
        - it runs `docker compose down --volumes`
        - but 'releases/docker-compose.yml' uses no volumes
        - db data exists on 'db/data/' directory
        - if you change to using docker volumes for db, this option may be useful
### db backup
* run 'DB_USER=postgres DB_NAME=app scripts/db-backup.sh [output_dir] [db_service_name]'
* default values are:
    - DB_USER=postgres
    - DB_NAME=app
    - [output_dir]=./backups
    - [db_service_name]=myapp_db
* `docker compose exec -T [db_service_name] pg_dump -U DB_USER -cC --column-inserts --if-exists DB_NAME`
* output to [output_dir]/db_[YYYY-MM-DD_hhmmss].sql
### db restore
* run 'DB_USER=postgres scripts/db-restore.sh <backup_file> [db_service_name]'
* <backup_file> must be referable from 'releases/current'
    - script wants to use 'docker compose' on 'releases/current/docker-compose.yml'
* default values are:
    - DB_USER=postgres
    - <backup_file>=./../backups/db_backup.sql
    - [db_service_name]=myapp_db
* `docker compose exec -T [db_service_name] psql -U DB_USER < releases/current/<backup_file>`
