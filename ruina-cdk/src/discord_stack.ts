import { Stack, StackProps } from "aws-cdk-lib";
import { Function } from "aws-cdk-lib/aws-lambda";
import { Construct } from "constructs";
import { DiscordBotConstruct } from "discord-bot-cdk-construct";

export class DiscordStack extends Stack {
    private readonly commandLambda: Function;
    private readonly discordBotConstruct: DiscordBotConstruct;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id);
        this.commandLambda = this.createCommandLambda();
        this.discordBotConstruct = this.createDiscordBotConstruct();
    }

    private createCommandLambda(): Function {
        throw new Error();
    }

    private createDiscordBotConstruct(): DiscordBotConstruct {
        return new DiscordBotConstruct(this, "DiscordBotConstruct", {
            commandsLambdaFunction: this.commandLambda,
        });
    }
}
