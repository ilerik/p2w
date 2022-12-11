import { Worker, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";
import { writeHeapSnapshot } from "v8";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();

  // Deploy contract
  const root = worker.rootAccount;
  const contract = await root.createSubAccount("test-account");
  const captainA = await root.createSubAccount("captain-a");
  const captainB = await root.createSubAccount("captain-b");

  // Get wasm file path from package.json test script in folder above
  await contract.deploy(process.argv[2]);

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, captainA, captainB };
});

test.afterEach.always(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("flow without dispute resolution", async (t) => {
  const { contract, root, captainA, captainB } = t.context.accounts;
  const game_id = await root.call(contract, "start_game", {
    team_a: [captainA],
    team_b: [captainB],
  });
  let game = await contract.view("get_game_by_id", { game_id });
  console.log(game);

  await captainA.call(contract, "finish_game", {
    game_id,
    result: "WinA",
  });
  game = await contract.view("get_game_by_id", { game_id });
  console.log(game);
  await captainB.call(contract, "finish_game", {
    game_id,
    result: "WinA",
  });
  game = await contract.view("get_game_by_id", { game_id });
  console.log(game);
  t.is(game_id, 0);
});

test("flow with dispute resolution", async (t) => {
  const { contract, root, captainA, captainB } = t.context.accounts;
  const game_id = await root.call(contract, "start_game", {
    team_a: [captainA],
    team_b: [captainB],
  });
  let game = await contract.view("get_game_by_id", { game_id });
  console.log(game);

  await captainA.call(contract, "finish_game", {
    game_id,
    result: "WinA",
  });
  game = await contract.view("get_game_by_id", { game_id });
  console.log(game);
  await captainB.call(contract, "finish_game", {
    game_id,
    result: "WinB",
  });
  game = await contract.view("get_game_by_id", { game_id });
  console.log(game);

  await contract.call(contract, "resolve_game", {
    game_id,
    result: "WinA",
  });
  game = await contract.view("get_game_by_id", { game_id });
  console.log(game);

  t.is(game_id, 0);
});
