import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createGrantPoolOutcome, sampleGrantPoolOutcome } from './common.js';

test('create GrantPoolOutcome', async () => {
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

    // Alice creates a GrantPoolOutcome
    const record: Record = await createGrantPoolOutcome(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read GrantPoolOutcome', async () => {
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

    const sample = await sampleGrantPoolOutcome(alice.cells[0]);

    // Alice creates a GrantPoolOutcome
    const record: Record = await createGrantPoolOutcome(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created GrantPoolOutcome
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pool_outcome",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the GrantPools for the new GrantPoolOutcome
    let linksToGrantPools: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pool_outcomes_for_grant_pool",
      payload: sample.grant_pool
    });
    assert.equal(linksToGrantPools.length, 1);
    assert.deepEqual(linksToGrantPools[0].target, record.signed_action.hashed.hash);
  });
});


