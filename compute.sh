#!/usr/bin/env bash

git clone "$REPO" repository --no-checkout
cd repository || return
../man-hours "$(git log | grep Date)"
