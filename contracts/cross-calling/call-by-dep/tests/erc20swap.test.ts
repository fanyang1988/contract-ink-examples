import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

async function deployConstract(contract_path, sender){
  const contractFactory = await getContractFactory(contract_path, sender);
  const contract = await contractFactory.deploy('new', '1000');
  const abi = artifacts.readArtifact(contract_path);

  return { contract, abi };
}

describe('ERC20Swap', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));

    // erc20
    const { contract:contractErc20, abi:abiErc20 } = await deployConstract('erc20', sender);
    console.log('erc20swap ', contractErc20.address.toString(), ' ', abiErc20)

    // erc20swap
    const { contract:contractSwap, abi:abiSwap } = await deployConstract('erc20swap', sender);
    console.log('erc20swap ', contractSwap.address.toString(), ' ', abiSwap)

    const receiver = await getRandomSigner();
    return { sender, contractSwap, abiSwap, receiver, Alice, one };
  }

  it('initial', async () => {
    await setup();
  });
});
