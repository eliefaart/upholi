# FROM nginx


# # Goal:
# # Docker container with NGINX server
# # requests to /api redirect to the server
# # requests to /, /albums, /etc to a static app file?
# # auto update letsencrypt certificate
# # external volume for image storage
# #
# # Get-Content Dockerfile | docker build -f- .
# # docker run -p 8080:80 1451c4c67e5e

# ENV HB_DB_CONNSTRING mongodb://localhost:27017/
# ENV HB_DB_NAME hummingbird-docker
# ENV HB_DIR_PHOTOS /home/photos

# COPY /docker/nginx.conf /etc/nginx/nginx.conf
# #COPY app/wwwroot /usr/share/nginx/html
# COPY app/wwwroot /home/app
# COPY target/debug/hummingbird /bin/

# EXPOSE 8000
# #EXPOSE 80


# RUN echo hello world
# #RUN /bin/hummingbird
# RUN echo end

# #CMD echo hello world
# #CMD ["./bin/hummingbird"]
# #CMD service nginx status
# #CMD echo end





FROM ubuntu


# Goal:
# Docker container with NGINX server
# requests to /api redirect to the server
# requests to /, /albums, /etc to a static app file?
# auto update letsencrypt certificate
# external volume for image storage
#
# Get-Content Dockerfile | docker build -f- .
# docker run -p 8080:80 1451c4c67e5e


# I may need multiple containers
# One NGINX, serving the app and redirecting to other container
# One for api, called from NGINX container, not exposed to public/


RUN apt-get update
RUN apt-get install -y nginx  
RUN rm -v /etc/nginx/nginx.conf

COPY /docker/nginx.conf /etc/nginx/
COPY app/wwwroot /home/app
COPY target/debug/hummingbird /bin/

#RUN echo "daemon off;" >> /etc/nginx/nginx.conf

ENV HB_DB_CONNSTRING mongodb://localhost:27017/
ENV HB_DB_NAME hummingbird-docker
ENV HB_DIR_PHOTOS /home/photos

EXPOSE 80
EXPOSE 8000

CMD ["nginx", "-g", "daemon off;"]
#CMD ["./bin/hummingbird"]
# CMD service nginx start

# CMD service nginx status
# CMD echo hello world