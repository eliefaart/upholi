FROM nginx

EXPOSE 80

# Copy app folder
COPY ./wwwroot /home/app

# Copy in default nginx configuration (which just forwards ACME requests to
# certbot, or redirects to HTTPS, but has no HTTPS configurations by default).
RUN rm -f /etc/nginx/conf.d/*
COPY ./docker/nginx.conf /etc/nginx/conf.d/