#!/usr/bin/env bash

cargo fmt && SQLX_OFFLINE=true cargo clippy
