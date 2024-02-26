import { derived, get, writable } from 'svelte/store';
import { AppAgentWebsocket } from '@holochain/client';

function holochainClientStore() {
  const isConnecting = writable(false);
  const client = writable<AppAgentWebsocket>();

  const { subscribe } = derived([isConnecting, client], ([$isConnecting, $client]) => ({
    client: $client,
    isConnecting: $isConnecting,
  }));
  
  async function connect(url: URL, happId: string) {
    if(get(client) !== undefined) return;
    
    const res = await AppAgentWebsocket.connect(url, happId);
    client.set(res);
  }

  return {
    subscribe,
    connect,
  };
}

export const holochainClient = holochainClientStore();