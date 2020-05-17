#!/usr/bin/env bash

git clone "$REPO" repository --no-checkout
cd repository || return
git log | grep 'Date'
../man-hours "$(git log | grep 'Date')"
