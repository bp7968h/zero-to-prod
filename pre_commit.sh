#!/usr/bin/env bash

cargo fmt --check && SQLX_OFFLINE=true cargo clippy -- -D warnings
