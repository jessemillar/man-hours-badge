#!/usr/bin/env bash

git clone "$1" repository --no-checkout
cd repository || return
../man-hours "$(git log | grep Date)"
