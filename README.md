# Imoyo
Crop white background, make image square format, exclude all EXIF

# Usage
Can process images in folder, single image (or link), multiple images (or links)

### Example

```sh
cargo run -- -cs ./path-to-image
```

This will crop all white or transparent background and make square image

## Arguments:

v - verbose mode.

c – crop.

s – square.

p – crop padding in pixels.

f – set [filter type](#filter-types) for image resizing (default Lanczos).

w – width of resized image.

a – apply alpha filter (exclude pixels with alpha less than filter value).

b – set background color (default white).

e - detect edges. Parameters: low_threshold, high_threshold. Saves image of detected edges in verbose mode.

### Example

```sh
cargo run -- -cp 10 ./path-to-image
```

Or

```sh
cargo run -- -e "5,50" ./path-to-image
```

This will crop all white or transparent background with padding of 10 pixels to an image. Will add white background if image does not have enough pigels for padding.

## Filter types

n – Nearest Neighbor.

t – Linear: Triangle.

c – Cubic: Catmull–Rom.

g – Gaussian.

l – Lanczos with window 3.

### Example

```sh
cargo run -- -f l -w 2000 ./path-to-image
```
