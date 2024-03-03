# Image Color quantization with 3D visuals
Takes all distinct colors of a given image and quantizes them into a smaller set of representative colors. This process reduces the number of colors in the image while preserving its visual appearance.

## Usage
After running the program, press the P key to recluster the state. Keep pressing P in order to 'quantize' the image and observe the visual changes in real-time.

### Controls
- Hold left mouse click and you can use WASD
- Hold right mouse click and move mouse to 'orbit'
- Mouse scroll to zoom in/out

### Other keybinds
O - toggle color mode

C - save current state of the image to disk

## Run command
```shell
cargo run -- -i <image> -k <desired_number_of_colors>
```

With default values
```shell
cargo run -- -i lena128.png -k 16
```

Help command
```shell
cargo run -- -help

Usage: image-quantization [OPTIONS]

Options:
  -i, --image <IMAGE_PATH>       [default: lena128.png]
  -k, --color <COLOR_COUNT>      Number of colors image will be quantized to [default: 16]
      --cube-size <CUBE_SIZE>    Size of a displayed cube [default: 0.25]
  -r, --radius <CLUSTER_RADIUS>  Radius of displayed cubes [default: 20.0]
  -h, --help                     Print help
```

### Preview
