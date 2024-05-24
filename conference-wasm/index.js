import init, { ConferenceManager } from './pkg/conference_wasm.js'

async function run() {
    await init();

    const conferenceManager = new ConferenceManager();
    await conferenceManager.sync();

    let questions = await conferenceManager.get();
    console.log("Messages are: ", questions);

    await conferenceManager.write("New Question!");

    questions = await conferenceManager.get();
    console.log("Messages are: ", questions);

    await conferenceManager.sync();
}

run()
