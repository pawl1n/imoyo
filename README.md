# imoyo
Crop white background, make image square format, exclude all EXIF

# Usage
Can process images in folder, single image (link), multiple images (links)

### Example

```sh
cargo run -- -cs
```

This will crop all white or transparent background and make suare image

## Arguments:

c - crop

s - square

p - crop padding in pixels

u - upscale image (allowed [filter types](#Filter-types): n, t, c, g, l)

e - change image extension (default jpg)

### Example

```sh
cargo run -- -cp 10
```

This will crop all white or transparent background with padding of 10 pixels to an image. Will add white background if image does not have enough pigels for padding

## Filter types

n - Nearest Neighbor

t - Linear: Triangle

c - Cubic: Catmull-Rom

g - Gaussian

l - Lanczos with window 3
