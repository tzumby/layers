defmodule Layers.MixProject do
  use Mix.Project

  @version "0.1.1"

  def project do
    [
      app: :layers,
      version: @version,
      elixir: "~> 1.14",
      description: description(),
      source_url: "",
      package: package(),
      name: "Layers",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  defp package do
    [
      maintainers: ["tzumby"],
      name: "layers",
      licenses: ["Apache License 2.0"],
      links: %{"GitHub" => "https://github.com/tzumby/layers"},
      files: [
        "mix.exs",
        "native/layers/src",
        "native/layers/.cargo/config.toml",
        "native/layers/Cargo.toml",
        "native/layers/Cargo.lock",
        "checksum-*.exs",
        "lib",
        "LICENSE",
        "README.md",
        "CHANGELOG.md"
      ]
    ]
  end

  defp description do
    "NIF that wraps an imaging library written in Rust, currently supports overlaying images"
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, "~> 0.4"},
      {:rustler, "~> 0.26.0"},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end
end
