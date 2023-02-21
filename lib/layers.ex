defmodule Layers do
  version = Mix.Project.config()[:version]

  @external_resource "README.md"
  @moduledoc "README.md"
             |> File.read!()

  use RustlerPrecompiled,
    otp_app: :layers,
    crate: "layers",
    base_url: "https://github.com/tzumby/layers/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_EXAMPLE_BUILD") in ["1", "true"],
    targets:
      RustlerPrecompiled.Config.default_targets() --
        ["aarch64-unknown-linux-musl", "x86_64-unknown-linux-musl"],
    version: version

  def from_bytes(_bytes), do: :erlang.nif_error(:nif_not_loaded)
  def layer_images(_bg_image, _fg_image, _x, _y), do: :erlang.nif_error(:nif_not_loaded)
end
