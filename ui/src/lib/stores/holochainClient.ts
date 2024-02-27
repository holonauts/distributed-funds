import { derived, get, writable } from 'svelte/store';
import { AppAgentWebsocket } from '@holochain/client';
import { toasts } from './toast';

function holochainClientStore() {
  const isConnecting = writable(true);
  const client = writable<AppAgentWebsocket>();

  const { subscribe } = derived([isConnecting, client], ([$isConnecting, $client]) => ({
    client: $client,
    isConnecting: $isConnecting,
  }));
  
  async function connect(url: URL, happId: string) {
    if(get(client) !== undefined) return;

    isConnecting.set(true);
    try {      
      const res = await AppAgentWebsocket.connect(url, happId);
      client.set(res);
    } catch(e) {
      toasts.error(`Failed to connect to holochain client: ${e as string}`);
    }
    isConnecting.set(false);
  }

  return {
    subscribe,
    connect,
  };
}

export const holochainClient = holochainClientStore();