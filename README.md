## Secure messaging

Proof of concept contracts for an on-chain secure messaging system, similar to an encrypted e-mail.
Users will register providing a public key (generated off-chain) which will be stored in a profiles contract. Other uses can obtain that public key to send messages previously encrypted with that public key for another user. Messages for a user can be queried, deleted and claimed (funds).

Workflow:

1. User A generates a private/public key pair off-chain or reuses one he already has.
2. User A creates a profile providing a user-id (nickname) and his public key. These 2 will be stored in a profile contract. The user-id is unique, only 1 person can register it, similar to a Name Service.
3. User B queries User A information and, using his public key, encrypts a message and sends it to the controller contract, that will route it to the User A "inbox". This message can have funds attached to it.
4. User A can: query the messages sent to him, claim funds from messages sent to him and delete any messages in his "inbox". When deleting a message, funds are automatically claim so that they are not lost.

To compile all contracts in the workspace deterministically, you can run:

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.13.0
```

To generate all the schema files run:

```bash
sh scripts/schema.sh 
```