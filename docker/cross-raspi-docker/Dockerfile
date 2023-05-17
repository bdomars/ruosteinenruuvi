FROM raspi-ubuntu-2204:latest

# Install libdbus libraries and pkg-config
RUN dpkg --add-architecture arm64	 && \
	    apt-get update && \
	    apt-get install --assume-yes libdbus-1-dev libdbus-1-dev:arm64 pkg-config libssl-dev:arm64