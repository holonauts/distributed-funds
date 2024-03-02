import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createGrantPool, sampleGrantPool } from './common.js';

test('create GrantPool', async () => {
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

    // Alice creates a GrantPool
    const record: Record = await createGrantPool(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read GrantPool', async () => {
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

    const sample = await sampleGrantPool(alice.cells[0]);

    // Alice creates a GrantPool
    const record: Record = await createGrantPool(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created GrantPool
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pool",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);

    // Bob gets the TimePeriods for the new GrantPool
    let linksToTimePeriods: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pools_for_time_period",
      payload: sample.time_period
    });
    assert.equal(linksToTimePeriods.length, 1);
    assert.deepEqual(linksToTimePeriods[0].target, record.signed_action.hashed.hash);
    // Bob gets the ApplicationTemplates for the new GrantPool
    let linksToApplicationTemplates: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pools_for_application_template",
      payload: sample.application_template
    });
    assert.equal(linksToApplicationTemplates.length, 1);
    assert.deepEqual(linksToApplicationTemplates[0].target, record.signed_action.hashed.hash);
    // Bob gets the EvaluationTemplates for the new GrantPool
    let linksToEvaluationTemplates: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_grant_pools_for_evaluation_template",
      payload: sample.evaluation_template
    });
    assert.equal(linksToEvaluationTemplates.length, 1);
    assert.deepEqual(linksToEvaluationTemplates[0].target, record.signed_action.hashed.hash);
  });
});


