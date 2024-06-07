import { Command } from "commander";
import * as dotenv from "dotenv";
import { DiscordAccessor, GlobalCommandList } from "./discord_accessor";
import * as commands_config from "./commands.json";

const program = new Command();
dotenv.config({ path: `${__dirname}/../.env` });

const applicationId: string = process.env.APPLICATION_ID!;
const botAuthToken: string = process.env.BOT_AUTH_TOKEN!;

program
  .version("1.0.0", "-v, --version")
  .usage("[OPTIONS]...")
  .option("-l, --list", "List all global commands in the bot currently.")
  .option("-w, --write", "Write and overwrite global commands stored in commands.json into the bot.")
  .option("-d, --delete <value>", "Delete specified global command id.")
  .parse(process.argv);

const options = program.opts();

// Since I'm too lazy to implement mutual exclusivity, the options are prioritized in this order
const isList: boolean = options.list;
const isWrite: boolean = options.write;
const deleteCommandId: string = options.delete;

const discordAccessor: DiscordAccessor = new DiscordAccessor({
    applicationId,
    botAuthToken
});

if (isList) {
    discordAccessor.getLiveGlobalCommands()
        .then((retval: GlobalCommandList) => {
            console.log(JSON.stringify(retval, null, 4));
        });
} else if (isWrite) {
    discordAccessor.writeGlobalCommands(commands_config as unknown as GlobalCommandList)
        .then((retval: GlobalCommandList) => {
            console.log(JSON.stringify(retval, null, 4));
        });
} else if (deleteCommandId) {
    // noop
    discordAccessor.deleteGlobalCommandById(deleteCommandId);
}
