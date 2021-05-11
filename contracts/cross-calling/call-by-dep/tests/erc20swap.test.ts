import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

async function deployERC20Constract(contract_path, sender){
  const contractFactory = await getContractFactory(contract_path, sender);
  const contract = await contractFactory.deploy('new', '1000000');
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
    const { contract:contractErc20, abi:abiErc20 } = await deployERC20Constract('erc20', sender);
    console.log('erc20swap ', contractErc20.address.toString(), ' ', abiErc20)

    // erc20swap
    const contractFactory = await getContractFactory('erc20swap', sender);
    const contract = await contractFactory.deploy('new', 'true', contractErc20.address.toString());
    const abi = artifacts.readArtifact('erc20swap');

    const receiver = await getRandomSigner();
    return { sender, contract, abi, contractErc20, receiver, Alice, one };
  }

  it('initial', async () => {
    const {sender, contract, abi, contractErc20, receiver, Alice, one} = await setup();
    const result = await contract.query.get();
    expect(result.output).to.equal(contractErc20.address);

    const resultBalance = await contractErc20.query.balanceOf(sender.address);
    expect(resultBalance.output).to.equal(1000000);

    const resultBalanceProxy = await contract.query.balanceOf(sender.address);
    expect(resultBalanceProxy.output).to.equal(1000000);
  });
});
