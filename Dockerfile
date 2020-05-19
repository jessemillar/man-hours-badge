FROM rust:1.40 as builder
WORKDIR /usr/src/man-hours
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine
RUN apk --update add git && \
	rm -rf /var/lib/apt/lists* && \
	rm /var/cache/apk/*
WORKDIR /man-hours-badge
COPY --from=builder /usr/local/cargo/bin/man-hours /usr/bin/man-hours
CMD "man-hours"
