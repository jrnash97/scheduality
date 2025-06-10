const net = require("node:net");
console.log("Connecting...");
const client = new net.Socket();
setTimeout(() => {
}, 1000);
client.connect({ port: 6379, host: "127.0.0.1", keepAlive: true }, () => {
  console.log("Connected!");
  client.destroy();
});
