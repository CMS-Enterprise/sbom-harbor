FROM public.ecr.aws/docker/library/rust:1.68.0-slim-bullseye as builder
WORKDIR /usr/src/harbor
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/harbor/target/release/harbor-api /usr/local/bin/harbor-api
CMD ["harbor-api"]
