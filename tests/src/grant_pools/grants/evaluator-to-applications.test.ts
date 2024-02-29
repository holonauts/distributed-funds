import { assert, test } from "vitest";

import { runScenario, dhtSync, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, Link, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createApplication } from './common.js';

test('link a Evaluator to a Application', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/grant-funding.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const baseAddress = alice.agentPubKey;
    const targetRecord = await createApplication(alice.cells[0]);
    const targetAddress = targetRecord.signed_action.hashed.hash;

    // Bob gets the links, should be empty
    let linksOutput: Link[] = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_applications_for_evaluator",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Evaluator to Application
    await alice.cells[0].callZome({
      zome_name: "grants",
      fn_name: "add_application_for_evaluator",
      payload: {
        base_evaluator: baseAddress,
        target_application_hash: targetAddress
      }
    });
    
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
    
    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_applications_for_evaluator",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(targetAddress, linksOutput[0].target);


    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "grants",
      fn_name: "get_evaluators_for_application",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 1);

  });
});

