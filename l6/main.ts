import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import { KeyringPair } from "@polkadot/keyring/types"
import { Data } from "@polkadot/types";
import { metadata } from "@polkadot/types/interfaces/essentials";

const WEB_SOCKET = 'ws://localhost:9944';
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const connectSubstrate = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider });
    await api.isReady;
    console.log("connection to substrate.");
    return api;
};

const getExistentialDeposit = async (api: ApiPromise) => {
    const deposit = await api.consts.balances.existentialDeposit.toHuman();
    console.log('const value existentialDeposit:', deposit);
    return deposit;
};

const bindAccount = async (api: ApiPromise, uri: string) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromUri(uri);
    await api.query.system.account(account.address, async (acct) => {
        const now = await api.query.timestamp.now();
        const acosub = acct.data.free;
        console.log(`${now} @`, uri, `balance:${acosub.toHuman()}`);
    });
};

const printBalance = async (uri: string, name: string, api: ApiPromise) => {
    console.log(`bind@[${name}]:`);
    await bindAccount(api, uri);
};

const getMetadata = async (api: ApiPromise) => {
    const data = await api.rpc.state.getMetadata();
    console.log('printMetadata:');
    console.log(data);
    return data;
};

const transferFormTo = async (from: string, to: string, api: ApiPromise, amount: Number) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const faccount = keyring.addFromUri(from);
    const taccount = keyring.addFromUri(to);
    console.log(`${from} to ${to} ${amount}`);
    await api.tx.balances.transfer(taccount.address, amount).signAndSend(faccount, res => {
        console.log(`Tx status:${res.status}`);
    });
};

const receiveEvent = async (api: ApiPromise) => {
    api.query.system.events((events) => {
        console.log(`\nReceived ${events.length} events:`);
        events.forEach((record) => {
            // Extract the phase, event and the event types
            const { event, phase } = record;
            if (event) {
                const types = event.typeDef;

                // Show what we are busy with
                console.log(` ${event.section}:${event.method}:: (phase=${phase.toString()})`);
                if (event.meta.documentation) console.log(`  ${event.meta.documentation.toString()}`);

                // Loop through each of the parameters, displaying the type and data
                event.data.forEach((data, index) => {
                    console.log(`  ${types[index].type}: ${data.toString()}`);
                });
            }
        });
    });
}

const main = async () => {
    console.log('main start');
    const api = await connectSubstrate();
    await getExistentialDeposit(api);
    //var metadata = await getMetadata(api);
    //printBalance('//Alice', 'alice', api);
    //printBalance('//Bob', 'bob', api);
    //await transferFormTo('//Alice', '//Bob', api, 123 ** 12);
    receiveEvent(api);

    var loop = true;
    const stdin = process.openStdin();
    stdin.addListener("data", function (inp) {
        var action = inp.toString().trim();
        if (action == 'q') {
            console.log("quitting...");
            loop = false;
        } else {
            console.log("enter q for quit! [" + action + "]");
        }
    });

    do {
        const delay = 1000;
        await sleep(delay);
        //console.log("SLEEP:", delay);
    } while (loop);
};

main().then(() => {
    console.log('success exited');
    process.exit(0);
}).catch(err => {
    console.log('error:', err);
    process.exit(1);
});