FROM scratch

ADD ./target/x86_64-unknown-linux-musl/release/coding-challenge /coding-challenge

ENV ROCKET_ENV=prod
ENTRYPOINT ["/coding-challenge"]
CMD []
