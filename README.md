# Debian-Based-NAPI-Rust-Addons-Wasm

<br>
<strong>
Create a Debian-Based NAPI-Rust Image and Container for Writing and Testing NAPI-Rust Addon Modules for Node.js Applications.
</strong>
<br><br>

NAPI-Rust is similar but different from  NAPI C/C++ (https://nodejs.org/api/n-api.html) 

NAPI-Rust is based on Rust toolchain and does not use node-gyp. <br> 

See NAPI C/C++ example: https://github.com/MongoExpUser/Shale-Reservoir-DNN-and-Drilling-Rare-Events-Graph/blob/master/test_addon.cc


## DEPLOYING IMAGE

### To deploy the image, follow these steps:

1) #### Clone repo
    Clone the repo. 
    The Repo contains the following files: <br>
    (1)  Dockerfile <br>
    (2)  Cargo.toml <br>
    (3)  package.json <br>
    (4)  index.js <br>
    (5)  ./src/build.rs <br>
    (6)  ./src/lib.rs <br>
    (7)  ./src/main.rs <br>
    (8)  ./src/utilities.rs <br>
    

    <strong> Modify </strong> the  above files as needed, or can keep the original contents to test.
    
2) #### BUILD docker image: 
   sudo docker build -t  mongoexpuser/napi-rust-app:latest .
   
3) #### RUN docker container and map local CWD to docker working directory: "/home/myapp" : 
   sudo docker run -itd --name running-napi-rust-app -v "$PWD":/home/myapp --workdir=/home/myapp --privileged --restart unless-stopped mongoexpuser/napi-rust-app:latest

4) #### INTERACT with container: start/restart, stop, shell into, and exit container instance, respectively: 
   sudo docker start running-napi-rust-app <br>
   sudo docker stop running-napi-rust-app <br>
   sudo docker exec -it running-napi-rust-app bash <br>
   exit
   
5) #### RUN commands inside the Docker instance, to compile, re-compile, run node.js app and clean code artifacts, respectively:
   sudo docker exec -it running-napi-rust-app npm run build <br>
   sudo docker exec -it running-napi-rust-app npm rebuild <br>
   sudo docker exec -it running-napi-rust-app node index.js <br>
   sudo docker exec -it running-napi-rust-app cargo clean <br>

6) ####  TEST Node.js code inside the container in the CWD:
   sudo docker exec -it running-napi-rust-app node index.js 
   
   

## MISCELLANEOUS

### Wasm:

1) In addition, the repo contains miscellaneous sample Rust codes and steps on how to compile the codes to <strong>WASM (Web Assembly) </strong> file. <br>
   See the link below. <br>
   https://github.com/MongoExpUser/Debian-Based-NAPI-Rust-Addons/blob/main/wasm/sample.rs

2) The repo also contains HTML file that uses the compiled <strong>WASM</strong> file on the Front-End (Browser). <br>
   See the link below. <br>
   https://github.com/MongoExpUser/Debian-Based-NAPI-Rust-Addons/blob/main/wasm/index.html
    

# References
1 - https://github.com/napi-rs/napi-rs <br>
2 - https://www.npmjs.com/package/@napi-rs/cli <br>



# License

Copyright © 2015 - present. MongoExpUser

Licensed under the MIT license.
