VERSION 0.8

IMPORT github.com/mortenlj/earthly-lib/rust/commands AS lib-commands
IMPORT github.com/mortenlj/earthly-lib/rust/targets AS lib-targets

FROM rust:1-bullseye

WORKDIR /code

chef-planner:
    FROM lib-targets+common-build-setup
    RUN apt-get --yes install libpam0g libpam0g-dev

    DO lib-commands+CHEF_PREPARE
    SAVE ARTIFACT recipe.json

build:
    FROM lib-targets+prepare-tier1
    RUN apt-get --yes install libpam0g libpam0g-dev

    COPY +chef-planner/recipe.json recipe.json

    DO lib-commands+BUILD --target x86_64-unknown-linux-gnu

    ARG version=unknown
    SAVE ARTIFACT target/x86_64-unknown-linux-gnu/release/libpam_bitwarden.so AS LOCAL target/libpam_bitwarden.${version}.so

    SAVE IMAGE --push ghcr.io/mortenlj/pam_bitwarden/cache:build
