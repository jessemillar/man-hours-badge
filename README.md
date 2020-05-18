# Man Hours

![Man Hours](https://img.shields.io/endpoint?url=https%3A%2F%2Fjessemillar-man-hours.herokuapp.com%2Fhours%3Frepo%3Dhttps%3A%2F%2Fgithub.com%2Fjessemillar%2Fman-hours.git)

## Overview

I want to have a badge that shows an estimate of how many man hours I've spent on a project. I'd like it to be as easy as adding the badge image to the README file and that's all the setup. I want to write any server-side code in Rust.

## Usage

1. Navigate to the [Man Hours Badge Generator](https://jessemillar.com/r/man-hours/generator)
1. Paste in the URI you'd use to clone your repository via HTTPS (e.g. `https://github.com/jessemillar/man-hours.git`)
1. Copy the generated Markdown and paste it into your README

## Notes

### Algorithm

The algorithm is very simple and focuses on getting a decent estimation instead of a fully accurate metric. It basically goes like this:

1. Run `git log`
1. Extract timestamps from the `git log` output
1. Iterate through the timestamps comparing each one to the one immediately previous
	- If the difference between the two timestamps is < 8 hours, add the time difference to the total man hours
	- If the difference is > 8 hours, add 1 hour to the total man hours (to account for design/ramp up time not represented by commit history)
1. Report back the total number of hours

### Heroku

You can easily deploy Man Hours to your own Heroku account if you'd like. You'll need to add a "Heroku Redis" add-on to your application post-deploy.

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

### Docker Hub

A Docker container housing the `man-hours` binary is continually built and accessible on [Docker Hub](https://hub.docker.com/repository/docker/jessemillar/man-hours).

### Miscellaneous

- My algorithm was inspired by [the `git-hours` project](https://github.com/kimmobrunfeldt/git-hours/blob/8aaeee237cb9d9028e7a2592a25ad8468b1f45e4/index.js#L114-L143).

## FAQ

### Why does my badge say "calculating"?

Larger repositories take a while to `git clone` even with the `--bare` argument. If you see "calculating" in place of a number on your badge, that means the backend service wasn't able to clone and parse your repo within 5 seconds. Don't worry, it's likely done calculating your man hour total. The [shields.io](https://shields.io/endpoint) has a mandatory cache time of 300 seconds so you'll have to wait that long and then refresh to see your correct total displayed on your badge.

Man Hours uses a Redis cache to prevent unnecessarily recalculating hour counts each time someone HTTP requests your badge. The cached hour totals never expire and automatically recompute (triggered via normal badge HTTP request) after 24 hours. If the cache goes down (for planned/unplanned maintenance), totals will have to be recalculated (which will happen upon the next badge HTTP request) and will show "calculating" during that process.
