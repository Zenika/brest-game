defmodule GameServer.Room.Room do
  def create(name) do
    %{
      id: Ecto.UUID.generate(),
      name: name
    }
  end
end
