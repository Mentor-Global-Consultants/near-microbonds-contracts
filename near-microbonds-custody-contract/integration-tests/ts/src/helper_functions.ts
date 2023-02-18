import { Worker, NearAccount } from 'near-workspaces';

export async function setupNFT(owner: NearAccount, custody_contract: NearAccount) {
    const nft_contract = await owner.devDeploy(
        './supplementary_contracts/bond_nft.wasm',
        {
            method: 'new_default_meta',
            args: {
                owner_id: owner.accountId
            },
        }
    );

    // Mint token to custody contract
    await owner.call(nft_contract, 'nft_mint', {
        token_id: '1',
        metadata: {
            title: 'Bond',
        },
        receiver_id: custody_contract.accountId,
    }, {
        attachedDeposit: '5990000000000000000000'
    });

    return nft_contract;
}