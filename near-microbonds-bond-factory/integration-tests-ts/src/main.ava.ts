import { Worker, NearAccount, AccountBalance, Account, NEAR } from 'near-workspaces';
import anyTest, { TestFn } from 'ava';
import fs from 'fs';

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

const INITIAL_BALANCE = '100000000000000000000000000000'; // 100,000 NEAR / 100,000^24 yoctoNEAR
const PRICE_PER_BYTE = '10000000000000000000'; // 10,000,000,000,000,000,000 yoctoNEAR

type TokenMetadata = {
    name: string;
    symbol: string;
};

test.beforeEach(async (t) => {
    // Init the worker and start the Sandbox server
    const worker = await Worker.init();

    // Create accounts and add balances
    const root = worker.rootAccount;
    const owner = await root.createSubAccount('owner', {
        initialBalance: INITIAL_BALANCE
    });

    const non_owner = await root.createSubAccount('non_owner', {
        initialBalance: INITIAL_BALANCE
    });
    
    // Get wasm file path from package.json test script in folder above
    const factory_contract = await owner.devDeploy(
        process.argv[2],
        {
            method: 'new',
            args: {
                owner_id: owner.accountId
            },
        }
    );

    // Save state for test runs, it is unique for each test
    t.context.worker = worker;
    t.context.accounts = { root, owner, factory_contract, non_owner };
});

test.afterEach.always(async (t) => {
    // Stop the Sandbox server
    await t.context.worker.tearDown().catch((error) => {
        console.log('Failed to stop Sandbox server', error);
    });
});


// === Testing adding token versions
test.failing('Should fail to add a new token version if not owner', async (t) => {
    const { factory_contract, non_owner } = t.context.accounts;

    const code = fs.readFileSync('./test-contracts/nft.wasm', 'utf8');
    const code_b64 = Buffer.from(code, 'base64');

    await non_owner.call(factory_contract, 'add_token_version', code_b64);
});

test('Should add a new token version if owner', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    const code = fs.readFileSync('./test-contracts/nft.wasm', 'utf8');
    const code_b64 = Buffer.from(code, 'base64');

    await owner.call(factory_contract, 'add_token_version', code_b64);

    const stored_versions: String[] = await factory_contract.view(
        'get_token_versions'
    );

    t.is(stored_versions.length, 1);
    t.is(stored_versions[0], '0');
});

test.failing('Should fail to add when the input is not passed', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_token_version', Uint8Array.from([]));
});


test('Should return the correct deployment cost', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    const code = fs.readFileSync('./test-contracts/nft.wasm', 'utf8');
    const code_b64 = Buffer.from(code, 'base64');

    await owner.call(factory_contract, 'add_token_version', code_b64);

    const cost = await factory_contract.view(
        'get_deployment_cost', {
            token_version: '0',
        }
    );

    t.is(cost, '5550000000000000000000');
});

// === Testing adding municipalities
test.failing('Should fail to add a new municipality if not owner', async (t) => {
    const { factory_contract, non_owner } = t.context.accounts;

    await non_owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });
});

test('Should add a new municipality if owner', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    const projects_for_municipality: String[] = await factory_contract.view(
        'view_projects_for_municipality', {
            municipality_id: 'test_municipality',
        }
    );

    t.is(projects_for_municipality.length, 0);
});

test.failing('Should fail to add a new municipality if the municipality already exists', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });
});

// === Testing adding projects to municipalities
test.failing('Should fail to add a new project if not owner', async (t) => {
    const { factory_contract, owner, non_owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    await non_owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });
});

test.failing('Should fail to add a new project if the given municipality does not exist', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });
});

test.failing('Should fail to add a new project if the project already exists', async (t) => {
    const { factory_contract, owner } = t.context.accounts;

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });

    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });
});

test('Should add a new project if owner', async (t) => {
    const {factory_contract, owner} = t.context.accounts;

    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });

    const projects_for_municipality: String[] = await factory_contract.view(
        'view_projects_for_municipality', {
            municipality_id: 'test_municipality',
        }
    );

    t.is(projects_for_municipality.length, 1);
    t.is(projects_for_municipality[0], 'test_project');
});

// === Testing adding tokens to projects
test.failing('Should fail to add a new token to project if not owner', async (t) => {
    const { factory_contract, owner, non_owner } = t.context.accounts;

    // Add token version
    const code = fs.readFileSync('./test-contracts/nft.wasm', 'utf8');
    const code_b64 = Buffer.from(code, 'base64');

    await owner.call(factory_contract, 'add_token_version', code_b64);

    // Add municipality
    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    // Add project
    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });

    await non_owner.call(factory_contract, 'add_new_token_for_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
        token_version: '0',
    });
});

test.failing('Should fail to add a new token if the given municipality does not exist', async (t) => {
    const { factory_contract, owner, non_owner } = t.context.accounts;

    await non_owner.call(factory_contract, 'add_new_token_for_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
        token_version: '0',
    });
});

test.failing('Should fail to add a new token if the given project does not exist', async (t) => {
    const { factory_contract, owner, non_owner } = t.context.accounts;

    // Add municipality
    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    await non_owner.call(factory_contract, 'add_new_token_for_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
        token_version: '0',
    });
});

test.failing('Should fail to add a new token if the given token version does not exist', async (t) => {
    const { factory_contract, owner, non_owner } = t.context.accounts;

    // Add municipality
    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    // Add project
    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });

    await non_owner.call(factory_contract, 'add_new_token_for_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
        token_version: '0',
    });
});


test('Should add a new token to project if owner', async (t) => {
    const { factory_contract, owner, root } = t.context.accounts;

    // Add token version
    const code = fs.readFileSync('./test-contracts/nft.wasm', 'base64');
    const code_b64 = Buffer.from(code, 'base64');

    await owner.call(factory_contract, 'add_token_version', code_b64);

    // Add municipality
    await owner.call(factory_contract, 'add_new_municipality', {
        municipality_id: 'test_municipality',
    });

    // Add project
    await owner.call(factory_contract, 'add_new_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
    });

    await owner.call(factory_contract, 'add_new_token_for_project', {
        municipality_id: 'test_municipality',
        project_id: 'test_project',
        token_version: '0',
        token_account_name: 'test_token',
        token_name: 'test_token',
        token_symbol: 'TT',
    }, {
        attachedDeposit: NEAR.from('3183350000000000000000000').add(NEAR.parse('1')).toString(),  // Cost of the contract = PRICE_PER_BYTES * size_of_contract_in_bytes
        gas: '300000000000000',
    });

    const tokens_for_project: string[] = await factory_contract.view(
        'view_tokens_for_project', {
            municipality_id: 'test_municipality',
            project_id: 'test_project',
        }
    );

    t.is(tokens_for_project.length, 1);
    
    const token_meta: TokenMetadata = await owner.call(tokens_for_project[0], 'nft_metadata', {});

    t.is(token_meta.name, 'test_token');
    t.is(token_meta.symbol, 'TT');
});