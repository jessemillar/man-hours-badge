FROM rust:1.40 as builder
WORKDIR /usr/src/man-hours
COPY . .
RUN cargo install --path .

FROM archlinux/base:latest
# Install dependencies
RUN pacman -Syy && yes | pacman -S git grep
ADD man-hours /
COPY --from=builder /usr/src/man-hours /compute.sh
COPY --from=builder /usr/local/cargo/bin/man-hours /man-hours
CMD ["/compute.sh"]
