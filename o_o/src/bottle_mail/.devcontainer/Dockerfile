FROM rust:1.77-slim-bookworm
RUN adduser rustuser -gecos "" --disabled-password \
&& apt-get update \
&& apt-get install -y build-essential git \
&& rm -fr /var/lib/apt/lists/*