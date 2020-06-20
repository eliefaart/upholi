FROM ubuntu
# Cannot use 'alpine' as base, it doesn't work out of the box. 
# Error I am getting is: standard_init_linux.go:211: exec user process caused "no such file or directory"

EXPOSE 8000
ARG DEBIAN_FRONTEND=noninteractive

# Install some dependencies
RUN apt-get update &&\
	apt-get -y install openssl &&\
	apt-get -y install pkg-config &&\
	apt-get -y install libssl-dev

# Copy app folder
COPY target/release/hummingbird /bin/

# Add execution rights and create folder that a volume will be mounted to
RUN chmod +x /bin/hummingbird &&\
	mkdir /srv/hummingbird

CMD ["./bin/hummingbird"]