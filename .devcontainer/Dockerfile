FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bullseye

# Install Deno
ENV DENO_INSTALL=/usr/local
RUN apt-get update && apt-get -y install cmake
RUN curl -fsSL https://deno.land/x/install/install.sh | sh

