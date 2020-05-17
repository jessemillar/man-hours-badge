FROM archlinux/base:latest

# Install dependencies
RUN pacman -Syy && yes | pacman -S git grep

ADD service /
CMD ["/service"]
