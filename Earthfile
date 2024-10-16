VERSION 0.8

IMPORT github.com/mortenlj/earthly-lib/rust/commands AS lib-commands
IMPORT github.com/mortenlj/earthly-lib/rust/targets AS lib-targets

FROM rust:1-bullseye

WORKDIR /code

chef-planner:
    FROM lib-targets+common-build-setup

    DO lib-commands+CHEF_PREPARE
    SAVE ARTIFACT recipe.json

build-target:
    FROM lib-targets+prepare-tier1

    COPY +chef-planner/recipe.json recipe.json

    ARG target
    DO lib-commands+BUILD --target ${target}

    ARG version=unknown
    SAVE ARTIFACT --if-exists target/${target}/release/libpam_bitwarden.so AS LOCAL target/libpam_bitwarden.so.${version}.${target}

    SAVE IMAGE --push ghcr.io/mortenlj/pam_bitwarden/cache:build-${target}

build:
    FOR target IN x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
        BUILD +build-target --target=${target}
    END
