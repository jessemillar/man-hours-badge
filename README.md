# Man Hours Badge

[![Man Hours](https://img.shields.io/endpoint?url=https%3A%2F%2Fmh.jessemillar.com%2Fhours%3Frepo%3Dhttps%3A%2F%2Fgithub.com%2Fjessemillar%2Fman-hours-badge.git)](https://jessemillar.com/r/man-hours)

## Overview

When skimming a new repository, I'm always curious how much time went into creating it. I built Man Hours to generate and display [a shields.io badge](https://shields.io) for your README with an estimate of how many hours committers have spent working on your files. You can see a sample badge above with the total hours put into this repo.

All the server-side code is written in Rust because that's the new hotness (rightly so from what I've seen).

## Usage

1. Navigate to the [Man Hours Badge Generator](https://jessemillar.com/r/man-hours-badge/generator)
1. Paste in the URI you'd use to clone your repository via HTTPS (e.g. `https://github.com/jessemillar/man-hours-badge.git`)
1. Copy the generated Markdown and paste it into your README

## Notes

### Algorithm

The algorithm is very simple and focuses on getting a decent estimation instead of a fully accurate metric. It was inspired by [the `git-hours` project](https://github.com/kimmobrunfeldt/git-hours) and basically goes like this:

1. Run `git log`
1. Extract timestamps from the `git log` output
1. Iterate through the timestamps comparing each one to the one immediately previous
	- If the difference between the two timestamps is < 8 hours, add the time difference to the total man hours
	- If the difference is > 8 hours, add 1 hour to the total man hours (to account for design/ramp up time not represented by commit history)
1. Report back the total number of hours

The algorithm only looks at the `master` branch and assumes a healthy, regular commit cadence.

### Heroku

You can easily deploy Man Hours to your own [Heroku](https://www.heroku.com/) account if you'd like. You'll need to add a "Heroku Redis" add-on to your application post-deploy.

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

### Docker Hub

A Docker container housing the `man-hours` binary is continually built and accessible on [Docker Hub](https://hub.docker.com/r/jessemillar/man-hours-badge). To run it outside the [Heroku](https://www.heroku.com/) platform, you'll need to set and pass in the `PORT` and `REDIS_URL` environment variables as seen in the command below:

```
docker run --rm -e PORT="$PORT" -e REDIS_URL="$REDIS_URL" -p $PORT:$PORT jessemillar/man-hours-badge:latest
```

## FAQ

### Why does my badge say "error"?

That means one of two things:

1. The repository URI you passed to the [Man Hours Badge Generator](https://jessemillar.com/r/man-hours-badge/generator) was malformed. Try generating it again.
1. The backend service encountered an issue while cloning/parsing your project that will likely resolve automatically. If it doesn't after a few hours, feel free to [file an issue](https://github.com/jessemillar/man-hours-badge/issues).

### Why does my badge say "calculating"?

Larger repositories take a while to `git clone` even with the `--bare` argument. If you see "calculating" in place of a number on your badge, that means the backend service wasn't able to clone and parse your repo within 5 seconds. Don't worry, it's likely done calculating your man hour total. The [shields.io](https://shields.io/endpoint) CDN has a mandatory cache time of 300 seconds so you'll have to wait that long and then refresh to see your correct total displayed on your badge.

Man Hours uses a Redis cache to prevent unnecessarily recalculating hour counts each time someone HTTP requests your badge. The cached hour totals never expire and automatically recompute (triggered via normal badge HTTP request) after 24 hours. If the cache goes down (for planned/unplanned maintenance), totals will have to be recalculated (which will happen upon the next badge HTTP request) and will show "calculating" during that process.

### Can I use this on a private repo?

Currently, no. Since the Man Hours Badge service clones the repo (at least the history) on the backend, you'd have to grant the service read permissions for your whole private repo via a token. This seems like poor security/privacy practice so I've opted to leave out that functionality for now. If there's significant demand later on, I'll consider alternative methods.
