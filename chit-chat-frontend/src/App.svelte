<script lang="ts">
  const chatServerSocket: WebSocket = new WebSocket("ws://127.0.0.1:8000/ws");

  let messages: string[] = [  ];
  let message: string = "";

  const sendMessageToChatServer = () => {
    console.log(`Sent: \`${message}\``);
    chatServerSocket.send(message);
  }

  chatServerSocket.addEventListener("message", (event) => {
    console.log(`Received: \`${event.data}\``);
    messages.push(event.data);
    messages = messages;
  });
</script>


<style>
  main {
    display: flex;
    gap: 1rem;
    flex-direction: column;
  }

  form input {
    width: 100%;
  }

  .message-container {
    background-color: rgb(16, 16, 16);
    overflow-x: hidden;
    overflow-y: auto;

    max-height: 50vh;
    min-height: 50vh;
    min-width: 50vw;
  }
</style>


<h1>chit-chat</h1>

<main>
  <div class="message-container">
    {#each messages as message }
      <p class="message">{message}</p>
    {/each}
  </div>

  <form on:submit|preventDefault={sendMessageToChatServer}>
    <input bind:value={message} placeholder="Input" />
  </form>
</main>