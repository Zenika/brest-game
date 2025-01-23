defmodule Server.Room do
  use GenServer
  require Logger

  def start_link(room_name) do
    GenServer.start_link(__MODULE__, room_name, name: via_tuple(room_name))
  end


  def create(_) do
    Ecto.UUID.generate()
  end

  def join(room_name, username) do
    GenServer.call(via_tuple(room_name), {:join, room_name, username})
  end

  def init(room_name) do
    Logger.info("Starting room process #{room_name}")
    {:ok, %{name: room_name, users: []}}
  end

  def handle_call({:join, room_name, username}, _, state) do
    Logger.info("User #{username} joined room #{room_name}, users are : #{Enum.join(state.users)}")
    {:reply, :ok, %{state | users: }} # TODO
  end

  def via_tuple(room_name), do: {:via, Registry, {Server.RoomRegistry, room_name}}

end
