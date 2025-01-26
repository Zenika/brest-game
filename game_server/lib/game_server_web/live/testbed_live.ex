defmodule GameServerWeb.Development.TestbedLive do
  alias GameServer.Development.ChannelsClient
  use GameServerWeb, :live_view

  def mount(_, _session, socket) do
    client =
      if connected?(socket) do
        {self(), uri: "ws://localhost:4000/socket/websocket?vsn=2.0.0"}
        |> ChannelsClient.start_link()
        |> case do
          {:ok, client} -> client
          {:error, {:already_started, client}} -> client
        end
      end

    socket =
      socket
      |> assign(:client, client)
      |> assign(:topics, [])
      |> assign(:join_form, to_form(%{}))
      |> assign(:push_form, to_form(%{}))
      |> stream(:channel_events, [], dom_id: &"#{&1.datetime}")

    {:ok, socket}
  end

  def handle_info(%{ref: ref, topic: topic, event: event, payload: payload}, socket) do
    event = incoming_event(ref, topic, event, payload)

    socket = stream_insert(socket, :channel_events, event)

    {:noreply, socket}
  end

  def handle_event("join", %{"topic" => topic}, socket) do
    event = outgoing_event(nil, topic, "join", nil)
    socket = stream_insert(socket, :channel_events, event)

    ChannelsClient.join(socket.assigns.client, topic)

    socket = update(socket, :topics, fn topics -> Enum.uniq([topic | topics]) end)

    {:noreply, socket}
  end

  def handle_event("push", %{"topic" => topic, "event" => event, "payload" => payload}, socket) do
    socket =
      payload
      |> JSON.decode()
      |> case do
        {:ok, payload} ->
          ref = ChannelsClient.push(socket.assigns.client, topic, event, payload)
          event = outgoing_event(ref, topic, event, payload)
          stream_insert(socket, :channel_events, event)

        {:error, _} ->
          put_flash(socket, :error, "Payload should be a valid JSON.")
      end

    {:noreply, socket}
  end

  defp outgoing_event(ref, topic, event, payload) do
    %{
      ref: ref,
      topic: topic,
      payload: payload,
      event: event,
      datetime: DateTime.utc_now(),
      direction: :outgoing
    }
  end

  defp incoming_event(ref, topic, event, payload) do
    %{
      ref: ref,
      topic: topic,
      payload: payload,
      event: event,
      datetime: DateTime.utc_now(),
      direction: :incoming
    }
  end
end
