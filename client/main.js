#!/usr/bin/env node

const process = require('process');
const http = require('http');
const fs = require('fs');


var helpMessage =
  `
fig-table Node.js CLI

Usage:
    fig-table <number>            Move table up by x centemeters
    fig-table [options]

Options:
    -h, --help                    Display this message
    -v, --version                 Print version info and exit
    -c, --config <ip-address>     Configure the app with a unique ip.`;

// Map of regex key to command function.
const commandMap = {

  '--help': () => {
    console.log(helpMessage)
  },

  '-h': () => commandMap['--help'](),

  '--version': () => {
    console.log(JSON.parse(fs.readFileSync('../package.json').version));
  },

  '-v': () => commandMap['--version'](),

  '--config': (args) => {
    if (args.length > 0 && args[1].match(/^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$/))
      return console.error('Error: Please enter a valid IPv4 address.');

    fs.writeFileSync('./config.json', JSON.stringify({ ip: args[0] }), { encoding: 'utf8' });

  },

  '-c': () => commandMap['--config'](),

  '(?:\d*\.)?\d+': (args) => {

    if (!fs.existsSync('./config.json'))
      return console.error('Error: Please configure application first (e.g. fig-table -c 255.255.255.255)');

    var {ip} = JSON.parse(fs.readFileSync('./config.json'));

    var postData = JSON.stringify({
      'vector': args[0],
      'time': 100
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
  }

}


// Start Processing Command Line Arguments

for (var i = 0; i < process.argv.length; i++) {

  // Check if there's a key in the Command Map 
  // that matches the command line argument
  var match = Object.keys(commandMap).reduce(
    (prev, cur) => prev | RegExp(cur).exec(process.argv[i]),
    null);

  if (match) {
    match([...process.argv].splice(0, i-1));
    break;
  }
  else if (i === process.argv.length - 1) {
    commandMap['-h']();
  }
}