/* 
 * This script should not be used stand-alone. 
 * It was designed to be used by main.rs to talk to Avanza.
*/

let Avanza = require('avanza')
const util = require('util')
const readline = require('readline')

switch(process.argv[2]) {
    case "totp":
        let totp_secret = require('avanza/dist/totp')(process.argv[3])
        console.log(totp_secret)
        process.exit()
    case "talk":
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        });
        const avanza = new Avanza()
        if(process.argv.length != 6) {
            console.error("Wrong number of arguments!")
            process.exit()
        }
        avanza.authenticate({
            totpSecret: process.argv[3],
            username: process.argv[4],
            password: process.argv[5]
        }).then(async () => {
            rl.on('line', async (line) => {
                const result = await handle_line_input(avanza, line);
                const raw_result = JSON.stringify(result)
                console.log(raw_result)
            })
        }).catch((e) => {
            console.error(e)
            process.exit()
        })
        break;
    case "test":
        test_suit();
        break;
    default:
        console.error("No mode argument passed! Choose between totp and talk.")
}

async function handle_line_input(avanza, input) {
    const parts = input.split(' ')
    let result = error("Unrecognizable command")
    if (parts.length == 0) return
    switch (parts[0]) {
        case "getpositions":
            result = ok(await avanza.getPositions())
            break;
        case "getinstrument":
            if (parts.length != 3)
                result = error("Not enought arguments!")
            result = ok(await avanza.getInstrument(parts[1], parts[2]))
            break;
        case "exit":
            process.exit()
    }
    return result
}

function error(description) {
    return {
        type: "error",
        description: description
    }
}

function ok(result) {
    return {
        type: "Ok",
        result: result
    }
}

async function test_suit() {
    const assert = require('assert')
    const mock_avanza = {
        getPositions: () => ok({}),
        getInstrument: (type, id) => ok({})
    };
    const result_positions = await handle_line_input(mock_avanza, "getpositions")
    assert(result_positions.type == "Ok")
    const result_instrument = await handle_line_input(mock_avanza, "getinstrument fund 2323")
    assert(result_instrument.type == "Ok")
    console.log("All is well!")
}