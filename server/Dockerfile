FROM ubuntu
# Cannot use 'alpine' as base, it doesn't work out of the box. 
# Error I am getting is: standard_init_linux.go:211: exec user process caused "no such file or directory"

EXPOSE 8000

# Copy app folder
COPY target/release/hummingbird /bin/
RUN chmod +x /bin/hummingbird

RUN mkdir /srv/hummingbird
RUN mkdir /srv/hummingbird/photos

CMD ["./bin/hummingbird"]