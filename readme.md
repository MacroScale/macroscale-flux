# MacroScale Flux

## Overview
An application designed to capture past n seconds of gameplay and upload it to 
a macroscale storage server. The application will have several features: 
- Capture the last n seconds of gameplay by hooking into the game's rendering 
  pipeline using various graphics APIs.
- Capture meta data about the game, such as the game's name, and the current 
  date and time. Will be used to categorise capture video on the website. 
- Upload the captured video to a macroscale storage server with minimal device
  storage needed (current uncaptured gameplay will be stored in memory.)
- Provide the user with a link to the uploaded video, which can be shared with
  others. It will be generated before the upload is complete, so the user can
  share it immediately.
- Allow the user to configure the capture settings, such as the length of the
  capture, quality, and whether to capture audio.

## Technologies
- Rust (for core application) - Will be used for the client side application
  that captures the gameplay and uploads it to the server.
- Golang (for webserver) - Will be used for the server side application that
  receives the uploaded video and stores it in a macroscale storage server. As 
  well as serving the website that displays the uploaded videos.

## Client
- The client will be a small application that runs in the background while the
  user is playing a game. It will hook into the game's rendering pipeline to
  capture the last n seconds of gameplay. The user will be able to configure
  the length of the capture, the quality, and whether to capture audio.

## Server/Website
- Have a website where users can view their uploaded videos, and share them
  with others. The website will be simple, with a search bar to find videos by
  game name, and a list of videos that the user has uploaded.

--- 

## Building the Community
-  Not sure yet lmao 
  
  
## Monitisation
- The application will be entirely free. The server will be funded by ads on
  the website, and by optional donations from users. The website will have a
  page where users can donate to the product, and the server will have a page
  where users can see how much has been donated, and how much is needed to keep
  the server running.

## Future Features
- Create a highlight system like steelseries' highlights, where the application
  will automatically detect interesting moments in the gameplay and create a
  highlight reel. The company wont be providing many of these highlight systems
  for games. Instead, the community will be able to create their own 
  highlights system via the plugin api (via lua scripts). There will be ample 
  documentation on how to create these scripts, with thorough examples.
- The ability to trim and make minor edits to the captured video after it has
  been uploaded.
