import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe('owner test', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('owner', sender);
    const contract = await contractFactory.deploy('new');
    const abi = artifacts.readArtifact('owner');

    return { sender, contractFactory, contract, abi, Alice, one };
  }

  it('Owner initial', async () => {
    const { contract, sender } = await setup();

    const result = await contract.query.getOwner();
    expect(result.output).to.equal(sender.address);
  });

  it('transfer ownership', async () => {
    const { contract, sender, Alice } = await setup();

    const result = await contract.query.getOwner();
    expect(result.output).to.equal(sender.address);

    await contract.tx.transferOwnership(Alice.address);

    const result1 = await contract.query.getOwner();
    expect(result1.output).to.equal(Alice.address);
  });
});
