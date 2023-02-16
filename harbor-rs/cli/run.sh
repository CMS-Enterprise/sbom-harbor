#!/bin/bash
clear && cargo build && ./target/debug/main -a 123456 --env blah start
