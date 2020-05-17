#!/usr/bin/env bash

git clone "$1" --no-checkout
git log | grep Date
./main
