# Debian-Based-NAPI-Rust-Addons

<br>
<strong>
Create a Debian-Based NAPI-Rust Image for Writing of and Testing Rust Addon Modules for Node.js Applications.
</strong>
<br><br>

Rust-NAPI is similar but different from  C/C++ Node-NAPI (https://nodejs.org/api/n-api.html) 

Rust-NAPI is based on Rust toolchain and does not use node-gyp. 


## DEPLOYING IMAGE

### To deploy the image, follow these steps:

1) #### Clone repo
    Clone the repo. 
    The Repo contains the following files: <br>
    (1)  Cargo.toml <br>
    (2)  package.json <br>
    (3)  index.js <br>
    (4)  ./src/build.rs <br>
    (5)  ./src/lib.rs <br>
    (6)  ./src/main.rs <br>

    <strong> Modify </strong> the  above files as needed, or can keep the original contents to test.
    
2) #### BUILD docker image: 
   sudo docker build -t  mongoexpuser/rust-napi-app:latest .
   
3) #### RUN docker container and map local CWD to docker working directory: "/home/myapp" : 
   sudo docker run -itd --name running-rust-napi-app -v "$PWD":/home/myapp --workdir=/home/myapp --privileged mongoexpuser/rust-napi-app:latest

4) #### Interacting with container: start/restart, stop, shell into, and exit container instance: 
   sudo docker start running-rust-napi-app <br>
   sudo docker stop running-rust-napi-app <br>
   sudo docker exec -it running-rust-napi-app bash <br>
   exit
   
5) #### Running command inside the Docker instance, to compile, re-compile, run node.js app and clean code artifacts:
   sudo docker exec -it running-rust-napi-app npm run build <br>
   sudo docker exec -it running-rust-napi-app npm rebuild <br>
   sudo docker exec -it running-rust-napi-app node index.js <br>
   sudo docker exec -it running-rust-napi-app cargo clean <br>

6) ####  Test Node.js code inside the container in the CWD:
   sudo docker exec -it running-rust-napi-app node index.js


# References
1 - https://github.com/napi-rs/napi-rs <br>
2 - https://www.npmjs.com/package/@napi-rs/cli <br>



# License

Copyright © 2015 - present. MongoExpUser
