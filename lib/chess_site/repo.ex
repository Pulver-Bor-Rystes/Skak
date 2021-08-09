defmodule ChessSite.Repo do
  use Ecto.Repo,
    otp_app: :chess_site,
    adapter: Ecto.Adapters.Postgres
end
