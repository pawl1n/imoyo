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

c – crop

s – square

p – crop padding in pixels

f – set [filter type](#filter-types) for image resizing (default Lancsoz)

w – width of resized image

e – change image extension (default JPG)

a – apply alpha filter (exclude pixels with alpha less than filter value)

b – set background color (default white)

### Example

```sh
cargo run -- -cp 10 ./path-to-image
```

This will crop all white or transparent background with padding of 10 pixels to an image. Will add white background if image does not have enough pigels for padding.

## Filter types

n – Nearest Neighbor

t – Linear: Triangle

c – Cubic: Catmull–Rom

g – Gaussian

l – Lanczos with window 3

### Example

```sh
cargo run -- -f l -w 2000 ./path-to-image
```
