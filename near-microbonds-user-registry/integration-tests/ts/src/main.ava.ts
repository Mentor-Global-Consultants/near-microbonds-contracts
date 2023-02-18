import { Worker, NearAccount } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
    // Init the worker and start the Sandbox server
    const worker = await Worker.init();
    const root = worker.rootAccount;

    // Deploy contracts
    const owner = await root.createSubAccount('owner', {
        initialBalance: '1000000000000000000000000000000'
    });
    
    // Get wasm file path from package.json test script in folder above
    const contract = await owner.devDeploy(
        process.argv[2],
        {
            method: 'new',
            args: {
                owner_id: owner.accountId,
            }
        }
    );

    // Save state for test runs, it is unique for each test
    t.context.worker = worker;
    t.context.accounts = { root, contract, owner };
});

test.afterEach(async (t) => {
    // Stop the Sandbox server
    await t.context.worker.tearDown().catch((error) => {
        console.log('Failed to stop Sandbox server', error);
    });
});

// Add user to municipality
test('Should fail to add user to a municipality if the caller is not the owner', async (t) => {
    const { contract, municipality } = t.context.accounts;
    await t.throwsAsync(async () => {
        await municipality.call(contract, 'add_user_to_municipality', {
            municipality_id: 'test-municipality',
            user_id: 'test-user',
        });
    });
});

test('Should add a user to a municipality if caller is owner', async (t) => {
    const { contract, owner } = t.context.accounts;
    await owner.call(contract, 'add_user_to_municipality', {
        municipality_id: 'test-municipality',
        user_id: 'test-user',
    });

    const users: string[] = await contract.view('get_users_for_municipality', {
        municipality_id: 'test-municipality'
    });
    t.is(users.length, 1);
    t.is(users[0], 'test-user');
});

// Check if user is in municipality
test('Should return true if a user is in a municipality', async (t) => {
    const { contract, owner } = t.context.accounts;
    await owner.call(contract, 'add_user_to_municipality', {
        municipality_id: 'test-municipality',
        user_id: 'test-user',
    });

    const userInMunicipality: boolean = await contract.view('is_user_in_municipality', {
        municipality_id: 'test-municipality',
        user_id: 'test-user'
    });
    t.is(userInMunicipality, true);
});

test('Should return false if a user is not in a municipality', async (t) => {
    const { contract } = t.context.accounts;
    const userInMunicipality: boolean = await contract.view('is_user_in_municipality', {
        municipality_id: 'test-municipality',
        user_id: 'test-user'
    });
    t.is(userInMunicipality, false);
});