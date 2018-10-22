/* 
 * This script should not be used stand-alone. 
 * It was designed to be used by main.rs to talk to avanza.
 * All verification of user input will be done there
 * and this script assumes that the correct arguments are passed.
*/

if (process.argv[2] === "totp") {
    let totp_secret = require('avanza/dist/totp')(process.argv[3])
    console.log(totp_secret)
    process.exit()
}

if (process.argv[2] == "positions") {
    let Avanza = require('avanza')
    const util = require('util')
    const avanza = new Avanza()

    avanza.authenticate({
        totpSecret: process.argv[3],
        username: process.argv[4],
        password: process.argv[5]
    }).then(async () => {
        const positions = await avanza.getPositions()
        console.log(util.inspect(positions, {showHidden: false, depth: null}))
        process.exit()
    }).catch((e) => {
        console.error(e)
        process.exit()
    })
}

if (process.argv[2] == "instrumentest") {
    let Avanza = require('avanza')
    const util = require('util')
    const avanza = new Avanza()

    avanza.authenticate({
        totpSecret: process.argv[3],
        username: process.argv[4],
        password: process.argv[5]
    }).then(async () => {
        const positions = await avanza.getInstrument("FUND", "471796")
        // keyRatios
        //     type: 'Aktiefond',
        console.log(util.inspect(positions, {showHidden: false, depth: null}))
        process.exit()
    }).catch((e) => {
        console.error(e)
        process.exit()
    })
}