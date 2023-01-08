defmodule Layers do
  @external_resource "README.md"
  @moduledoc "README.md"
             |> File.read!()

  use Rustler, otp_app: :layers

  def from_bytes(_bytes), do: :erlang.nif_error(:nif_not_loaded)
  def layer_images(_bg_image, _fg_image, _x, _y), do: :erlang.nif_error(:nif_not_loaded)
end
