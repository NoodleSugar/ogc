FROM ubuntu:noble

ENV DEBIAN_FRONTEND=noninteractive

RUN	apt update -y \
&&	apt install -y --no-install-recommends \
		sudo ssh git bash-completion ca-certificates \
		clang rustup \
&&	echo "ubuntu ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers.d/user

USER ubuntu
RUN	rustup default stable \
&&	rustup toolchain add nightly-x86_64-unknown-linux-gnu
