# ***************************************************************************************************************************************
# * Dockerfile                                                                                                                          *
#  **************************************************************************************************************************************
#  *                                                                                                                                    *
#  * @License Starts                                                                                                                    *
#  *                                                                                                                                    *
#  * Copyright © 2015 - present. MongoExpUser.  All Rights Reserved.                                                                    *
#  *                                                                                                                                    *
#  * License: MIT - https://github.com/MongoExpUser/Debian-Based-NAPI-Rust-Addons/blob/main/LICENSE                                     *
#  *                                                                                                                                    *
#  * @License Ends                                                                                                                      *
#  **************************************************************************************************************************************
# *                                                                                                                                     *
# *  Project: Rust Container Project for NA                                                                                             *
# *                                                                                                                                     *
# *  This dockerfile creates an image based on:                                                                                         *
# *                                                                                                                                     *                                                                                                              *
# *   1)  Rust:1.62.0 (Debian based)                                                                                                    *
# *                                                                                                                                     *
# *   2)  Additional debain Packages                                                                                                    *
# *                                                                                                                                     *
# *   3)  Python v3.9                                                                                                                   *
# *                                                                                                                                     *
# *   4)  Python3-pip                                                                                                                   *
# *                                                                                                                                     *
# *   5)  Boto3                                                                                                                         *
# *                                                                                                                                     *
# *   6)  Python's awscli upgrade                                                                                                       *
# *                                                                                                                                     *
# *   7)  Node.js v18x and @napi-rs/cli package                                                                                         *
# *                                                                                                                                     *
# *                                                                                                                                     *
# ***************************************************************************************************************************************


# base image
# Version -> Any of: 1-bullseye, 1.62-bullseye, 1.62.0-bullseye, bullseye, 1, 1.62, 1.62.0, latest
# Ref: https://hub.docker.com/_/rust
FROM rust:1.62.0

# labels
LABEL maintainer="MongoExpUser"
LABEL maintainer_email="MongoExpUser@domain.com"
LABEL company="MongoExpUser"
LABEL version="1.0"

# create user and add it to the sudoers group
RUN apt-get update && apt-get -y install sudo
RUN adduser --disabled-password --gecos 'rust-app' adminuser
RUN adduser adminuser sudo
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# create working directory and copy files
RUN cd /home/
RUN mkdir myapp
WORKDIR /home/myapp
COPY . .

# 1. install rust packages
RUN cargo install --path .
RUN cargo install cargo-rebuild

#  2. additional debian packages
RUN apt-get -y update && apt-get -y upgrade && apt-get -y dist-upgrade \
    && apt-get install -y systemd procps nano apt-utils wget curl gnupg2 make \
    && apt-get install -y sshpass cmdtest spamassassin snap nmap net-tools \
    && apt-get install -y aptitude build-essential gcc snapd screen spamc parted  \
    && apt-get install -y iputils-ping certbot python3-certbot-apache \
    #  3. python3 v.9
    && apt-get -y install python3.9  \
    #  4. python3-pip
    && apt-get -y install python3-pip \
    #  5. boto3
    && python3 -m pip install boto3 \
    #  6. upgrade python's awscli package
    && python3 -m pip install --upgrade awscli \
    #  7. node.js v18x
    && curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && echo -e "Y" \
    && apt-get install -y nodejs \
    && echo -e "Y" \
    && npm install @napi-rs/cli \
    && echo -e "Y" \
    #  finally, clean-up
    && rm -rf /var/lib/apt/lists/* \
    && apt-get autoclean \
    && apt-get autoremove
