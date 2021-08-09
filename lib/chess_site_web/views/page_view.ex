defmodule ChessSiteWeb.PageView do
  use ChessSiteWeb, :view


  def svelte(name, props) do
    raw """
    <div 
      class="svelte-component"
      data-name=#{name}
      data-props=#{json(props)}
      >
    </div>
    """
  end


  defp json(props) do
    props
    |> Poison.encode
    |> case do
      {:ok, message} ->
        message
      {:error, reason} ->
        IO.inspect(reason)
        ""
    end
  end
end
