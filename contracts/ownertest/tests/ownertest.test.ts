import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe('owner-test', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('owner_test', sender);
    const contract = await contractFactory.deploy('new', true);
    const abi = artifacts.readArtifact('owner_test');
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice, one };
  }

  it('Default initial', async () => {
    const { contract, sender } = await setup();
    const result = await contract.query.get();
    expect(result.output).to.equal(true);
  });

  it('Flip the status', async () => {
    const { contract, receiver } = await setup();
    await contract.tx.flip()

    const result1 = await contract.query.get();
    expect(result1.output).to.equal(false);

    await contract.tx.flip()

    const result2 = await contract.query.get();
    expect(result2.output).to.equal(true);
  });
});
