import { derived, get, writable } from 'svelte/store';
import { AppAgentWebsocket, type AgentPubKey } from '@holochain/client';
import { toasts } from './toast';
import { ProfilesClient, ProfilesStore } from '@holochain-open-dev/profiles';

function holochainClientStore() {
  const isConnecting = writable(true);
  const client = writable<AppAgentWebsocket>();
  const profilesStore = writable<ProfilesStore>();

  const { subscribe } = derived([isConnecting, client, profilesStore], ([$isConnecting, $client, $profilesStore]) => ({
    client: $client,
    profilesStore: $profilesStore,
    isConnecting: $isConnecting,
  }));
  
  function setupProfileStore() {
    profilesStore.set(new ProfilesStore(new ProfilesClient(get(client), 'grant_pools'), {
      avatarMode: 'avatar-optional',
      additionalFields: [
        {
          label: 'About Me',
          name: 'bio',
          required: true,
        },
        {
          label: 'Email',
          name: 'email',
          required: false,
        },
        {
          label: 'Website',
          name: 'website',
          required: false,
        },
        {
          label: 'LinkedIn',
          name: 'linkedin',
          required: false,
        },
        {
          label: 'X',
          name: 'x',
          required: false,
        },

      ]
    }));
  }

  
  async function connect(url: URL, happId: string) {
    if(get(client) !== undefined) return;

    isConnecting.set(true);
    try {      
      const res = await AppAgentWebsocket.connect(url, happId);
      client.set(res);
      await setupProfileStore();
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