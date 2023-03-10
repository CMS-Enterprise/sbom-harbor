#!/bin/bash
clear
cargo build && ../target/debug/main --account 123456  --env none start
