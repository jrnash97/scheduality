const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const { Client, Collection, Events, GatewayIntentBits, MessageFlags } = require('discord.js');
const { DatabaseSync } = require('node:sqlite');
const dotenv = require('dotenv');

dotenv.config();
const dcClient = new Client({ intents: [GatewayIntentBits.Guilds] });

dcClient.once(Events.ClientReady, readyClient => console.log(`${readyClient.user.tag} Logged in!`));
dcClient.on(Events.GuildCreate, guild => {
    let createGuildEntity = http.request({
        port: 8080,
        path: "/api/entities/create",
        headers: {
            "content-type": "text/html",
            "content-length": 0
        },
        method: "POST"
    }, (res) => {
        let rawData = '';
        res.on('data', (chunk) => { rawData += chunk });
        res.on('end', () => {
            console.log(rawData);
        });
    });
    // createGuildEntity.write("");
    // createGuildEntity.end();

});

dcClient.login(process.env.DISCORD_TOKEN);

dcClient.commands = new Collection();

const foldersPath = path.join(__dirname, 'commands');
const commandFolders = fs.readdirSync(foldersPath);

for (const folder of commandFolders) {
    const commandsPath = path.join(foldersPath, folder);
    const commandFiles = fs.readdirSync(commandsPath).filter(file => file.endsWith('.js'));
    for (const file of commandFiles) {
        const filePath = path.join(commandsPath, file);
        const command = require(filePath);
        // Set a new item in the Collection with the key as the command name and the value as the exported module
        if ('data' in command && 'execute' in command) {
            dcClient.commands.set(command.data.name, command);
        } else {
            console.log(`[WARNING] The command at ${filePath} is missing a required "data" or "execute" property.`);
        }
    }
}

dcClient.on(Events.InteractionCreate, async interaction => {
    if (!interaction.isChatInputCommand()) return;

    const command = dcClient.commands.get(interaction.commandName);
    if (!command) {
        console.error(`No command matching ${interaction.commandName}`);
        return;
    }
    try {
        await command.execute(interaction);
    } catch (error) {
        console.error(error);
        if (interaction.replied || interaction.deferred) {
            await interaction.followUp({ content: 'There was an error while executing this command!', flags: MessageFlags.Ephemeral });
        } else {
            await interaction.reply({ content: 'There was an error while executing this command!', flags: MessageFlags.Ephemeral });
        }
    }
})

