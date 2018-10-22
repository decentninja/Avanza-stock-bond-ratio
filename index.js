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
    const avanza = new Avanza()

    avanza.authenticate({
        totpSecret: process.argv[3],
        username: process.argv[4],
        password: process.argv[5]
    }).then(async () => {
        const positions = await avanza.getPositions()
        console.log(positions)
        process.exit()
    }).catch((e) => {
        console.error(e)
        process.exit()
    })
}