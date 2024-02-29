import { createWeb3Modal, defaultWagmiConfig } from '@web3modal/wagmi';
import { reconnect } from '@wagmi/core';
import { WALLETCONNECT_PROJECT_ID, CHAIN } from '$lib/config';

const metadata = {
  name: 'Distributed Funds',
  description: 'Peer to peer grant pools.',
  url: '',
  icons: []
}

export const config = defaultWagmiConfig({
  chains: [CHAIN],
  projectId: WALLETCONNECT_PROJECT_ID,
  metadata,
})
reconnect(config)

export const web3Modal = createWeb3Modal({
  wagmiConfig: config,
  projectId: WALLETCONNECT_PROJECT_ID,
  enableAnalytics: true // Optional - defaults to your Cloud configuration
})