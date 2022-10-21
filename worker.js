import init, { add } from './target/learn_worker.js';
console.log('worker.js init');

async function init_worker() {
    await init();
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