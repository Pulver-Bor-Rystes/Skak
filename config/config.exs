# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :chess_site,
  ecto_repos: [ChessSite.Repo]

config :chess_site, :pow,
  user: ChessSite.Users.User,
  repo: ChessSite.Repo

# Configures the endpoint
config :chess_site, ChessSiteWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "cwawtXYH7BF2ErN1Mak4V36STivthZQKqCLJXOHmoegnGVOO/2NSyWaGPgPRvgSb",
  render_errors: [view: ChessSiteWeb.ErrorView, accepts: ~w(html json), layout: false],
  pubsub_server: ChessSite.PubSub,
  live_view: [signing_salt: "wPThfll8"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
