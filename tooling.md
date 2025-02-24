# Build Tools

The build tools is a collation of various tools used to build the applications.
The tools are configure and used from the docker environment.

## Usage:

1. build the tool image (only need to once)
    - ``` ./docker/docker-init.sh ```
2. run the ``` ./docker-build.sh ``` script located in each project folder
    - This mounts the folder into a folder called src in the container. From there
    you can use the ./build.sh to build the project

## Future
- streamline the building so that you dont need to enter the interactive terminal
of the container, it will just simply build in inside the container for you
