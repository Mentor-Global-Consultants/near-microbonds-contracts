import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';
import { setupNFT } from './helper_functions';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
    // Init the worker and start the Sandbox server
    const worker = await Worker.init();

    // Create accounts and add balances
    const root = worker.rootAccount;
    const owner = await root.createSubAccount('owner', {
        initialBalance: '10000000000000000000000000000000'
    });
    
    const user1_external_wallet_a = await root.createSubAccount('user1_a', {
        initialBalance: '10000000000000000000000000000000'
    });
    const user1_external_wallet_b = await root.createSubAccount('user1_b', {
        initialBalance: '10000000000000000000000000000000'
    });
    
    // Get wasm file path from package.json test script in folder above
    const custody_contract = await owner.devDeploy(
        process.argv[2],
        {
            method: 'new',
            args: {
                owner_id: owner.accountId
            },
        }
    );

    // Deploy nft contract and mint token to custody contract - !not saved to custody contract yet!
    const nft_contract = await setupNFT(owner, custody_contract);

    // Save state for test runs, it is unique for each test
    t.context.worker = worker;
    t.context.accounts = { root, owner, custody_contract, user1_external_wallet_a, user1_external_wallet_b, nft_contract };
});

test.afterEach(async (t) => {
    // Stop the Sandbox server
    await t.context.worker.tearDown().catch((error) => {
        console.log('Failed to stop Sandbox server', error);
    });
});

// === Test adding token to owner
test('Should fail to add token to owner if caller is not custody contract owner', async (t) => {
    const { custody_contract, nft_contract } = t.context.accounts;
    
    await t.throwsAsync(async () => {
        await nft_contract.call(custody_contract, 'add_new_token_for_owner', {
            owner_id: 'user1',
            token_account_id: nft_contract.accountId,
            token_id: '1',
        })
    });
});

test('Should add token to correct owner', async (t) => {
    const { custody_contract, owner, nft_contract } = t.context.accounts;
    
    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    const tokens: string = await custody_contract.view('tokens_for_owner', {
        owner_id: 'user1',
    });

    t.is(tokens[0], `${nft_contract.accountId}:1`);
});

test('Should fail to add an existing token', async (t) => {
    const { custody_contract, owner, nft_contract } = t.context.accounts;
    
    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    await t.throwsAsync(async () => {
        await owner.call(custody_contract, 'add_new_token_for_owner', {
            owner_id: 'user1',
            token_account_id: nft_contract.accountId,
            token_id: '1',
        })
    });
});

test('Should view tokens with a limit', async (t) => {
    const { custody_contract, owner, nft_contract } = t.context.accounts;
    
    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });
    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '2',
    });
    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '3',
    });

    const tokens: string[] = await custody_contract.view('tokens_for_owner', {
        owner_id: 'user1',
        limit: 2
    });

    t.is(tokens.length, 2);
});

test('Should view tokens with a limit and offset', async (t) => {
    const { custody_contract, owner, nft_contract } = t.context.accounts;

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '2',
    });

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '3',
    });

    const tokens: string[] = await custody_contract.view('tokens_for_owner', {
        owner_id: 'user1',
        from_index: '1',
        limit: 2
    });

    t.is(tokens.length, 2);
    t.is(tokens[0], `${nft_contract.accountId}:2`);
    t.is(tokens[1], `${nft_contract.accountId}:3`);
});


// === Test account linking and withdrawal
test('Should link an account to a user', async (t) => {
    const { custody_contract, owner, user1_external_wallet_a } = t.context.accounts;
    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_a.accountId,
    });
    const account: string = await custody_contract.view('get_account_for_user', {
        user_id: 'user1'
    });
    t.is(account, user1_external_wallet_a.accountId);
});

test('Should change the account for a user', async (t) => {
    const { custody_contract, owner, user1_external_wallet_a, user1_external_wallet_b } = t.context.accounts;
    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_a.accountId,
    });

    let account: string = await custody_contract.view('get_account_for_user', {
        user_id: 'user1'
    });
    t.is(account, user1_external_wallet_a.accountId);

    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_b.accountId,
    });

    account = await custody_contract.view('get_account_for_user', {
        user_id: 'user1'
    });
    t.is(account, user1_external_wallet_b.accountId);
});

// test('Fails to withdraw token to owner if caller is not the same as the linked account', async () => {
//     // TODO
// });
test('Should fail to withdraw if no account is linked', async (t) => {
    const { custody_contract, owner, nft_contract } = t.context.accounts;

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    await t.throwsAsync(async () => {
        await owner.call(custody_contract, 'send_token_to_owner', {
            owner_id: 'user1',
            token_account_id: nft_contract.accountId,
            token_id: '1',
        });
    });
});

test('Should fail to withdraw if the caller is not the linked account', async (t) => {
    const { custody_contract, owner, nft_contract, user1_external_wallet_a } = t.context.accounts;

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_a.accountId,
    });

    await t.throwsAsync(async () => {
        await owner.call(custody_contract, 'send_token_to_owner', {
            owner_id: 'user1',
            token_account_id: nft_contract.accountId,
            token_id: '1',
        });
    });
});

test('Should fail to withdraw if the caller is does not own the given token', async (t) => {
    const { custody_contract, owner, nft_contract, user1_external_wallet_a } = t.context.accounts;

    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_a.accountId,
    });

    await t.throwsAsync(async () => {
        await owner.call(custody_contract, 'send_token_to_owner', {
            owner_id: 'user1',
            token_account_id: nft_contract.accountId,
            token_id: '1',
        });
    });
});

test('Should withdraw the token to the owner if the caller is the linked account', async (t) => {
    const { custody_contract, owner, nft_contract, user1_external_wallet_a } = t.context.accounts;

    await owner.call(custody_contract, 'add_new_token_for_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    });

    await owner.call(custody_contract, 'link_account_to_user', {
        user_id: 'user1',
        account_id: user1_external_wallet_a.accountId,
    });

    await user1_external_wallet_a.call(custody_contract, 'send_token_to_owner', {
        owner_id: 'user1',
        token_account_id: nft_contract.accountId,
        token_id: '1',
    }, {
        gas: '45000000000000'
    });

    const tokens: string[] = await custody_contract.view('tokens_for_owner', {
        owner_id: 'user1',
    });

    t.is(tokens.length, 0);
});