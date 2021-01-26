FROM ubuntu
# Cannot use 'alpine' as base, it doesn't work out of the box.
# Error I am getting is: standard_init_linux.go:211: exec user process caused "no such file or directory"

EXPOSE 8000
ARG DEBIAN_FRONTEND=noninteractive

# Install some dependencies
# and ca-certificates, otherwise oauth requests to identity provider (at least to github.com) get rejected due to untrusted certificates
RUN apt-get update &&\
	apt-get -y install openssl &&\
	apt-get -y install ca-certificates

# Copy app folder
COPY target/release/upholi /bin/

# Copy configurations
COPY config/ /config/

# Add execution rights and create folder that a volume will be mounted to
RUN chmod +x /bin/upholi &&\
	mkdir /srv/upholi

CMD ["./bin/upholi"]