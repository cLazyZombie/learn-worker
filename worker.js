importScripts('./target/learn_worker.js');
console.log('worker.js init');

const { add } = wasm_bindgen;

async function init_worker() {
    await wasm_bindgen('./target/learn_worker_bg.wasm');
    console.log('worker.js init done');

    self.onmessage = async event => {
        console.log('worker receive message', event.data);

        let i = parseInt(event.data);
        let added = await add(i, i);

        console.log('worker sending added value', added);

        self.postMessage(added);
    };
};

init_worker();