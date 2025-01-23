defmodule ServerWeb.RoomChannel do
  use ServerWeb, :channel

  @impl true
  def join("room", payload, socket) do
    if authorized?(payload) do
      {:ok, socket}
    else
      {:error, %{reason: "unauthorized"}}
    end
  end

  @impl true
  def handle_in("join", %{"room_id" => room_id}, socket) do
    # Join the room
    {:reply, {:ok, "Room joined"}, socket}
  end

  # Add authorization logic here as required.
  defp authorized?(_payload) do
    true
  end


end
