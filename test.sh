#!/bin/sh

cargo test --test=all -- --color=never -n "$1"
