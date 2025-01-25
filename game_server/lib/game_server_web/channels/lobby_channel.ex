defmodule GameServerWeb.LobbyChannel do
  use Phoenix.Channel

  alias GameServer.Room.Room

  def join("lobby", _message, socket) do
    {:ok, socket}
  end

  def handle_in("create_room", %{"name" => name}, socket) do
    room = Room.create(name)
    broadcast!(socket, "new_room_created", room)
    {:reply, {:ok, room}, socket}
  end

  def handle_in("ping", payload, socket) do
    {:reply, {:ok, payload}, socket}
  end
end
