# hex_rbg_converter

The Hex-RGB Converter is a Rust library that allows you to convert colors between hexadecimal (hex) and RGB formats. It provides convenient macros and structs to simplify color conversion operations.

## Usage

To use the Hex-RGB Converter, you need to import the necessary modules and macros into your Rust project:

```rust
use hex_rgb_converter::{RgbColor, HexColor, Color};
#[macro_use] mod macros;
```

The Color struct and macros are provided by the library to facilitate color conversions.

### Examples

Here are some examples of how you can use the Hex-RGB Converter:

#### Multiple Ways to Instantiate Colors

There are multiple ways to instantiate colors using the Hex-RGB Converter:

```rust
// all the following lines instantiate the same struct (HexColor)
let hex_1: HexColor = Color::hex("fff");
let hex_2 = HexColor::new("fff");
let hex_3 = hex!("fff");
let hex_4 = color!("fff");

// all the following lines instantiate the same struct (RgbColor)
let rgb_1: RgbColor = Color::rgb(1, 2, 3);
let rgb_2 = RgbColor::new(1, 2, 3);
let rgb_3 = rgb!(1, 2, 3);
let rgb_4 = color!(1, 2, 3);
```

You can either or not add '#' at the start of the hex colors.
For simplicity purposes, we will be using macros to instantiate colors in the tutorial.

#### Convert Hex to RGB

To convert a color from hex format to RGB, you can use the color! macro followed by the hex value:

```rust
let my_hex_color: HexColor = color!("787878");
let my_color_in_rgb: RgbColor = my_hex_color.to_rgb();
println!("Hex Color: {}", my_hex_color);
println!("RGB Color: {}", my_color_in_rgb);
```

#### Convert RGB to Hex

To convert a color from RGB to hex format, you can use the hex! macro followed by the RGB values:

```rust
let test_color: HexColor = hex!("#12ef78").to_rgb().to_hex();
test_color.print();
```

#### Manipulate RGB Colors

You can also manipulate RGB colors using the provided methods. Here's an example:

```rust
let mut rgb_col: RgbColor = color!(23, 2, 255);
rgb_col
    .set_blue(90)
    .set_green(90)
    .set_red(90)
    .set_color(Colors::All(120));
// use Colors enum to choose the color (Reg,Green,Blue,All)
//can also use 'r', 'g' and 'b'

println!("RGB Color: {}", rgb_col);
println!("Hex Color: {}", rgb_col.to_hex());
```

#### Comparison and Equality

You can compare colors for equality using the `is_equal` method for different color formats and `RgbColor::are_equal` as well as `HexColor::are_equal` methods for the same format. Here's an example:

```rust
let hex_col: HexColor = hex!("#fff");
let rgb_col: RgbColor = rgb!(2, 4, 5);
println!("Are hex_col and rgb_col equal? {}", hex_col.is_equal(&rgb_col)); // false here

let other_hex: HexColor = hex!("ffffff");
println!("Are hex_col and other_hex equal? {}", HexColor::are_equal(&hex_col, &other_hex)); // true here
```

#### Color by Name

The Hex-RGB Converter also provides a convenient method to get the hex representation of a color by its name:

```rust
Color::by_name("orange").print();
```

## Contributing

If you'd like to contribute to the Hex-RGB Converter library, feel free to submit issues or pull requests on the [GitHub repository](https://github.com/DevYatsu/hex_rgb_converter).

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
