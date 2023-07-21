import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";
import { LorCommand } from "./lor_command";

export class CommandManager {
    private readonly lorCommand: LorCommand;

    constructor(lorCommand: LorCommand) {
        this.lorCommand = lorCommand;
    }

    public async routeRequest(request: Request): Promise<CommandResult> {
        switch (request.command) {
            // TODO: consider moving to enum
            case "lor":
                console.log(`CommandManager: Routing request to LOR`);
                return await this.lorCommand.invoke(request);
            default:
                console.log(`CommandManager: Did not find route for ${request.command}`);
                return {
                    success: false,
                    error: `Command for ${request.command} not found`,
                };
        }
    }
}
