defmodule GameServerWeb.LobbyChannel do
  use GameServerWeb, :channel

  alias GameServer.Room.Room

  @impl true
  def join("lobby", payload, socket) do
    if authorized?(payload) do
      {:ok, socket}
    else
      {:error, %{reason: "unauthorized"}}
    end
  end

  # Channels can be used in a request/response fashion
  # by sending replies to requests from the client
  @impl true
  def handle_in("ping", payload, socket) do
    {:reply, {:ok, payload}, socket}
  end

  # It is also common to receive messages from the client and
  # broadcast to everyone in the current topic (lobby).
  @impl true
  def handle_in("shout", payload, socket) do
    broadcast(socket, "shout", payload)
    {:noreply, socket}
  end

  def handle_in("create_room", %{"name" => name}, socket) do
    {:ok, room} = Room.create(name)
    broadcast!(socket, "new_room_created", room)
    {:reply, {:ok, room}, socket}
  end

  # Add authorization logic here as required.
  defp authorized?(_payload) do
    true
  end
end
