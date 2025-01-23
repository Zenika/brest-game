defmodule Server.Room.RoomSupervisor do
  use DynamicSupervisor

  def start_link(_args), do: DynamicSupervisor.start_link(__MODULE__, [], name: __MODULE__)

  def init(_init_arg), do: DynamicSupervisor.init(strategy: :one_for_one)

  def start_table_child(table_name) do
    DynamicSupervisor.start_child(__MODULE__, %{
      id: Room,
      start: {Table, :start_link, [table_name]},
      restart: :transient
    })
  end

end
