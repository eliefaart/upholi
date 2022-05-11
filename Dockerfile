FROM ubuntu:20.04
# Cannot use 'alpine' as base, it doesn't work out of the box.
# Error I am getting is: standard_init_linux.go:211: exec user process caused "no such file or directory"

EXPOSE 8000

ARG DEBIAN_FRONTEND=noninteractive

ENV UPHOLI_SERVER_WWWROOT_PATH="/srv/upholi-app"

# Install some dependencies
# and ca-certificates, otherwise oauth requests to identity provider (at least to github.com) get rejected due to untrusted certificates
RUN apt-get update &&\
	apt-get -y install openssl &&\
	apt-get -y install ca-certificates

# Copy app folder
COPY ./app/wwwroot /srv/upholi-app

# Copy serve executable folder
COPY ./server/target/release/upholi /bin/

# Copy configurations
COPY ./server/config/ /config/

# Add execution rights and create folder that a volume will be mounted to
RUN chmod +x /bin/upholi &&\
	mkdir /srv/upholi

CMD ["./bin/upholi"]