import { ApiPromise, WsProvider } from "@polkadot/api";
import {hexToString} from "@polkadot/util";

const WEB_SOCKET = 'ws://localhost:9944';

async function main() {
    const wsProvider = new WsProvider(WEB_SOCKET);
    // 拿到api句柄
    const api = await ApiPromise.create({ provider: wsProvider});
    await api.isReady;
    console.log('connection to substrate is OK');

    // ocw-demo::storage::tx\u{8}\0\0\0
    // const storage = await api.rpc.offchain.localStorageGet(
    //     "PERSISTENT",
    //     "ocw-demo::storage::tx"
    // );
    const storage = await api.rpc.offchain.localStorageGet(
        "PERSISTENT",
        "ocw-demo::storage::tx\u{8}\0\0\0"
    );

    const hexValue = storage.toHex();
    console.log("hexValue: ", hexValue);
    let stringValue = hexToString(hexValue);
    console.log("value in offchain storage: ", stringValue);
}

main()
    .then(() => {
        console.log("success");
        process.exit(0);
    })
    .catch(err => {
        console.log('error', err);
        process.exit(1);
    })