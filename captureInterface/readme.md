# Macroscale Game Capturer

## Dependencies
- mingw-w64-cppwinrt (this will install mingw, wine as well as the winrt headers)
- cmake

## Build

** you may have to <code>chmod +x build.sh</code> first **

```

cd client_backend
./build.sh

```

## Future
- create build image (docker)
    - this is the ensure that mingw-w64-cppwinrt, cmake, etc.. have been correctly installed
    allowing for platform agnostic builds
    - Haven't looked into it yet, but hopefully there is a way to create a oneshot image where
    it caches the installed deps to the container and then a simple call can be made for building 
