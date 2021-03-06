FROM rust:buster
#FROM debian

## Install Base Packages
RUN apt-get update && apt-get -y install \
    apache2 \
    make \
    curl \
    git \
    gcc

RUN a2enmod rewrite
RUN a2enmod cgid
RUN a2enmod cgi

RUN ln -sf /proc/self/fd/1 /var/log/apache2/access.log && \
    ln -sf /proc/self/fd/1 /var/log/apache2/error.log && \
    ln -sf /proc/self/fd/1 /var/log/apache2/other_vhosts_access.log


RUN echo '<VirtualHost *:80> \n\
   ServerAdmin webmaster@myawesomesite.com \n\
   DocumentRoot /var/www/html/ \n\
   Options +ExecCGI \n\
   DirectoryIndex web \n\
   <FilesMatch "^[^\.]+$"> \n\
     SetHandler cgi-script \n\
   </FilesMatch> \n\
   <Directory "/var/www/html/"> \n\
       SetEnvIf Content-Type "(.+)" HTTP_CONTENT_TYPE=$1 \n\
       SetEnvIf Authorization "(.+)" HTTP_AUTHORIZATION=$1 \n\
       PassEnv IS_AWS_LAMBDA DATABASE_URI \n\
       Options +ExecCGI \n\
       <FilesMatch "^[^\.]+$"> \n\
         SetHandler cgi-script \n\
       </FilesMatch> \n\
       <IfModule mod_rewrite.c> \n\
           RewriteEngine On \n\
           RewriteBase / \n\
           RewriteRule ^web$ - [L] \n\
           RewriteCond %{REQUEST_FILENAME} !-f \n\
           RewriteCond %{REQUEST_FILENAME} !-d \n\
           RewriteRule . /web [L] \n\
        </IfModule> \n\
   </Directory> \n\
</VirtualHost> \n\
' > /etc/apache2/sites-available/000-default.conf

RUN mkdir /app

COPY ./ /app/
COPY ./storage /var/www/html/storage

RUN sh -c "cd /app; cargo build; cp -r ./target/debug/* /var/www/html/;"

EXPOSE 80

CMD /usr/sbin/apache2ctl -D FOREGROUND
