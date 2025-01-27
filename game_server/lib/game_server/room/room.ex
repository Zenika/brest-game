defmodule GameServer.Room.Room do
  def create(name) do
    {:ok,
     %{
       id: Ecto.UUID.generate(),
       name: name
     }}
  end
end
