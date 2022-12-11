# P2W related NEAR contracts

NEAR contract designed to govern play2win game flow with dispute resolution and automatic rewards distribution.

To build and deploy contract run:

```bash
npm run install
npm run build
npm run deploy
```

## Deployment status:

- NEAR testnet: [p2w-v1.ilerik.testnet](https://explorer.testnet.near.org/accounts/p2w-v1.ilerik.testnet)
- NEAR mainnet: TBD

## Game flow

Game lifecycle diagram (italic for contract calls):

_create_game( reward, team_A, team_B )_

|

Ongoing // Game has started

|

_finish_game(outcome)_

|

// Captains declared same outcomes game is resolved automatically

Finished { outcome } --> _finish_game( result == outcome )_ --> Resolved { outcome }

|

_finish_game( result != outcome )_

|

// Captains declared different outcomes admins need to resolve manually

Disputed-->_resolve_game(outcome)_-->Resolved { outcome }

To run [integration tests](/integration-tests/src/main.ava.ts) covering the above flows:

```bash
npm run test:integration
```

## TO DO:

- ability to manage community of administrators responsible for dispute resolution
- implement reward distribution (either immediate on resolution or via claim mechanics or both)
