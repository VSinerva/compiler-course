FROM rust AS builder
WORKDIR /home/vili/School/Compilers/compiler-course
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
RUN apt-get update && apt-get install -y libc6 gcc && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/compiler-course /usr/local/bin/compiler-course
EXPOSE 3000
CMD ["compiler-course"]

