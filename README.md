# Image thumbnailer

## Description

Utility for image thumbnails creating in the loop.

## Usage

### Arguments

Utility can be execute only with all arguments or without them.

1. Source folder - the folder where the source images will be placed. Default is `./images`
2. Target folder - the folder where the created thumbnails will be placed. Default is `./thumbnails`
3. Sizes - list of sizes devided by `,` char. Default is `240,480`

### Example

`image_thumbnailer`

It runs loop that will convert all images from `./images` folder to `./thumbnails` folder(include folders for each size) and delete source images.

`image_thumbnailer images/wallpapers my_thumbs 240,480,960`

It runs loop that will convert all images from `./images/wallpapers` folder to `./my_thumbs` folder(include folders for each size) and delete source images.
