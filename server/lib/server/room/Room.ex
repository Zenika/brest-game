defmodule Server.Room do
  def create(_) do
    Ecto.UUID.generate()
  end
end
