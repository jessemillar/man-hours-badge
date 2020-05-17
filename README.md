# Man Hours

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

I want to have a badge that shows an estimate of how many man hours I've spent on a project.

Something like this: ![Man Hours](https://img.shields.io/badge/Man%20Hours-777-yellow)

I'd like it to be as easy as adding the badge image to the README file and that's all the setup

I want to write any server-side code in Rust

If I'm running on Heroku, I'll definitely need to enable HTTP caching (probably only clone the repo once every day or so to save dyno hours): https://devcenter.heroku.com/articles/http-caching#enabling-http-caching

Running `git clone <repo> --no-checkout` pulls git history but no files (which should be faster)

Docker container is built and accessible here: https://hub.docker.com/repository/docker/jessemillar/man-hours

I need a way of only calculating hours for approved repos (maybe have an env var that's a GitHub username and only clone if it matches)

Inspired by [this project](https://github.com/kimmobrunfeldt/git-hours/blob/8aaeee237cb9d9028e7a2592a25ad8468b1f45e4/index.js#L114-L143) but I hate Node and refuse to use it
