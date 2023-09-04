import { BinahBotCommand } from "../command/binahbot_command";
import { LorCommand } from "../command/lor_command";
import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";

export class CommandRouter {
    private readonly lorCommand: LorCommand;
    private readonly binahbotCommand: BinahBotCommand;

    // TODO: figure out a way to put this in a prop object
    // Can't do so right now due to inability to mock dependencies
    // (Seems to be a jest.Mock + Typescript limitation?)
    constructor(lorCommand: LorCommand, binahbotCommand: BinahBotCommand) {
        this.lorCommand = lorCommand;
        this.binahbotCommand = binahbotCommand;
    }

    public routeRequest(request: Request): CommandResult {
        console.log(
            `RequestRouter: received request ${JSON.stringify(request)}`
        );
        switch (request.command) {
            // TODO: consider moving to enum
            case "lor":
                console.log(`RequestRouter: Routing request to LOR command`);
                return this.lorCommand.invoke(request);
            case "binahbot":
                console.log(
                    `RequestRouter: Routing request to BinahBot command`
                );
                return this.binahbotCommand.invoke(request);
            default:
                console.log(
                    `RequestRouter: Did not find route for ${request.command}`
                );
                return {
                    success: false,
                    error: `Command for ${request.command} not found`,
                };
        }
    }
}
