FROM mcr.microsoft.com/vscode/devcontainers/rust:1-bullseye

# Install Deno
ENV DENO_INSTALL=/usr/local
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
RUN apt-get update && apt-get -y install cmake libfontconfig-dev

