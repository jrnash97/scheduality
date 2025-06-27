const { SlashCommandBuilder } = require("discord.js");

module.exports = {
    data: new SlashCommandBuilder().setName('ping').setDescription('Say "Pong"!'),
    async execute(interaction) {
        console.log(`${interaction.user.tag} pinged Scheduardo`);
        await interaction.reply("Pong!");
    }
}
