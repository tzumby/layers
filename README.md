# Layers

This library is heavily inspired by [mirage](https://github.com/scrogson/mirage) 
and provides a Rust NIF that wraps the `image` crate and currently supports overlaying images.

## Installation

This library requires the Rust toolchain to compile.

### Install rust

```
curl https://sh.rustup.rs -sSf | sh
```

### Add package

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `layers` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:layers, "~> 0.1.0"}
  ]
end
```

## Usage

```
raw_bg_image = File.read!("/path/to/bg.png")
raw_fg_image = File.read!("/path/to/fg.png")

{:ok, bg_image} = Layers.from_bytes(raw_bg_image)
{:ok, fg_image} = Layers.from_bytes(raw_fg_image)

{:ok, bytes, _image} = Layers.layer_images(bg_image.data, fg_image.data, _x = 0, _y = 0)

File.write!("output.png", bytes)

```

