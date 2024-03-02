import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, Link, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createGrantPool } from './common.js';

test('link a Sponsor to a GrantPool', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/distributed-funds.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const baseAddress = alice.agentPubKey;
    const targetRecord = await createGrantPool(alice.cells[0]);
    const targetAddress = targetRecord.signed_action.hashed.hash;

    // Bob gets the links, should be empty
    let linksOutput: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pools_for_sponsor",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Sponsor to GrantPool
    await alice.cells[0].callZome({
      zome_name: "grants",
      fn_name: "add_grant_pool_for_sponsor",
      payload: {
        base_sponsor: baseAddress,
        target_grant_pool_hash: targetAddress
      }
    });
    
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
    
    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pools_for_sponsor",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(targetAddress, linksOutput[0].target);


    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_sponsors_for_grant_pool",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 1);

  });
});

