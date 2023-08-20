import { BinahBotCommand } from "../command/binahbot_command";
import { LorAutocomplete } from "../command/lor_autocomplete";
import { LorCommand } from "../command/lor_command";
import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";

export class RequestRouter {
    private readonly lorCommand: LorCommand;
    private readonly lorAutocomplete: LorAutocomplete;
    private readonly binahbotCommand: BinahBotCommand;

    // TODO: figure out a way to put this in a prop object
    // Can't do so right now due to inability to mock dependencies
    // (Seems to be a jest.Mock + Typescript limitation?)
    constructor(
        lorCommand: LorCommand,
        lorAutocomplete: LorAutocomplete,
        binahbotCommand: BinahBotCommand
    ) {
        this.lorCommand = lorCommand;
        this.lorAutocomplete = lorAutocomplete;
        this.binahbotCommand = binahbotCommand;
    }

    public routeRequest(request: Request): CommandResult {
        console.log(
            `RequestRouter: received request ${JSON.stringify(request)}`
        );
        switch (request.command) {
            // TODO: consider moving to enum
            case "lor":
                if (request.autocomplete) {
                    console.log(
                        `RequestRouter: Routing request to LOR autocomplete`
                    );
                    return this.lorAutocomplete.autocomplete(request);
                } else {
                    console.log(
                        `RequestRouter: Routing request to LOR command`
                    );
                    return this.lorCommand.invoke(request);
                }
            case "binahbot":
                if (!request.autocomplete) {
                    console.log(
                        `RequestRouter: Routing request to BinahBot command`
                    );
                    return this.binahbotCommand.invoke(request);
                }
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
