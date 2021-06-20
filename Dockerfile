FROM rust:alpine
#RUN apk --no-cache add curl
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y
COPY . /app
WORKDIR /app
RUN rustup default nightly
CMD cargo run --release