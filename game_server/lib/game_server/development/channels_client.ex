defmodule GameServer.Development.ChannelsClient do
  @behaviour GenServer

  use Slipstream, restart: :temporary

  def start_link(args) do
    Slipstream.start_link(__MODULE__, args, name: __MODULE__)
  end

  @impl Slipstream
  def init({suscriber, config}) do
    socket =
      config
      |> connect!()
      |> assign(:suscriber, suscriber)
      |> assign(:refs, Map.new())

    {:ok, socket}
  end

  @impl Slipstream
  def handle_connect(socket) do
    {:ok, notify_subscriber(socket, nil, nil, "connected", nil)}
  end

  @impl Slipstream
  def handle_disconnect(reason, socket) do
    {:stop, :normal, notify_subscriber(socket, nil, nil, "diconnected", %{reason: reason})}
  end

  @impl Slipstream
  def handle_join(topic, payload, socket) do
    {:ok, notify_subscriber(socket, nil, topic, "join.ok", payload)}
  end

  @impl Slipstream
  def handle_message(topic, event, payload, socket) do
    {:ok, notify_subscriber(socket, nil, topic, event, payload)}
  end

  @impl Slipstream
  def handle_reply(ref, {status, payload}, socket) do
    socket =
      case Map.pop(socket.assigns.refs, ref, nil) do
        {{topic, event}, refs} ->
          socket
          |> notify_subscriber(ref, topic, "#{event}.#{status}", payload)
          |> assign(:refs, refs)

        {nil, _} ->
          socket
          |> notify_subscriber(nil, nil, "#{:join}.#{status}", payload)
      end

    {:ok, socket}
  end

  def handle_reply(ref, status, socket), do: handle_reply(ref, {status, nil}, socket)

  defp notify_subscriber(socket, ref, topic, event, payload) do
    send(socket.assigns.suscriber, %{
      ref: ref,
      topic: topic,
      event: event,
      payload: payload
    })

    socket
  end

  @impl GenServer
  def handle_cast({:join, topic}, socket) do
    {:noreply, Slipstream.join(socket, topic)}
  end

  @impl GenServer
  def handle_call({topic, event, payload}, _from, socket) do
    {:ok, ref} = Slipstream.push(socket, topic, event, payload)

    socket = update(socket, :refs, fn refs -> Map.put_new(refs, ref, {topic, event}) end)

    {:reply, ref, socket}
  end

  def join(client, topic) do
    GenServer.cast(client, {:join, topic})
  end

  def push(client, topic, event, payload) do
    GenServer.call(client, {topic, event, payload})
  end
end
