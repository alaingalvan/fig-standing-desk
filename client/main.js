#!/usr/bin/env node

const process = require('process');
const http = require('http');
const fs = require('fs');
const path = require('path');

const installPath = path.join(__filename, '..');

const helpMessage =
  `
🍐 fig-table Node.js CLI

Usage:
    fig-table <number>            Move table up by x centimeters
    fig-table [options]

Options:
    -h, --help                    Display this message
    -v, --version                 Print version info and exit
    -c, --config <ip-address>     Configure the app with a unique ip.`;

// Map of regex key to command function.
const commandMap = {

  '--help': () => {
    console.log(helpMessage);
  },

  '-h': () => commandMap['--help'](),

  '--version': () => {
    console.log(JSON.parse(fs.readFileSync('../package.json').version));
  },

  '-v': () => commandMap['--version'](),

  '--config': (args) => {
    if (!(args.length > 0 && args[1] && args[1].match(/(((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?))/)))
      return console.error('Error: Please enter a valid IPv4 address.');
    
    fs.writeFileSync(installPath + '/config.json', JSON.stringify({
      ip: args[1]
    }), {
      encoding: 'utf8'
    });

    console.log('✅ Configuration stored @ ' + installPath)
  },

  '-c': (args) => commandMap['--config'](args),

  '[-.0-9]+': (args) => {

    if (!fs.existsSync(installPath + '/config.json'))
      return console.error('Error: Please configure application first (e.g. fig-table -c 255.255.255.255)');

    var {
      ip
    } = JSON.parse(fs.readFileSync(installPath + '/config.json'));

    let cmPerSecond = 0.5;
    let direction = args[0] > 0 ? .9 : -.9;
    let time = Math.abs(args[0]) * cmPerSecond * 1000;

    var postData = JSON.stringify({
      direction,
      time
    });

    var options = {
      hostname: ip,
      port: 80,
      path: '/api',
      method: 'POST',
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
        'Content-Length': Buffer.byteLength(postData)
      }
    };

    var req = http.request(options, (res) => {

      res.setEncoding('utf8');

      res.on('data', (chunk) => {
        console.log(`${chunk}`);
      });
    });

    req.on('error', (e) => {
      console.log(`problem with request: ${e.message}`);
    });

    // write data to request body
    req.write(postData);
    req.end();
    console.log('↕️️ Moving table %s %s cm.', direction > 0 ? 'up' : 'down', Math.abs(args[0]));
  }
}

// Start Processing Command Line Arguments
if (process.argv.length < 3)
      commandMap['-h']();

for (let command in commandMap)
  if (RegExp(command).test(process.argv[2])) 
    return commandMap[command]([...process.argv].splice(2, process.argv.length));