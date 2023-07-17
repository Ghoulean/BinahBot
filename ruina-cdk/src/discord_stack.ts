import { Duration, Stack, StackProps } from "aws-cdk-lib";
import { Code, Function, Runtime } from "aws-cdk-lib/aws-lambda";
import { BlockPublicAccess, Bucket } from "aws-cdk-lib/aws-s3";
import { Construct } from "constructs";
import { DiscordBotConstruct } from "discord-bot-cdk-construct";

export class DiscordStack extends Stack {
    private readonly commandLambda: Function;
    private readonly discordBotConstruct: DiscordBotConstruct;
    private readonly imageHostBucket: Bucket;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id);
        this.commandLambda = this.createCommandLambda();
        this.discordBotConstruct = this.createDiscordBotConstruct();
        this.imageHostBucket = this.createImageHostBucket();
        this.commandLambda.addEnvironment("S3_BUCKET_NAME", this.imageHostBucket.bucketName);
    }

    private createCommandLambda(): Function {
        return new Function(this, "DiscordBotFunction", {
            runtime: Runtime.NODEJS_18_X,
            handler: "build/src.handler",
            code: Code.fromAsset("../ruina-discord-bot/ruina-discord-bot.zip"),
            timeout: Duration.seconds(30),
            memorySize: 512,
        });
    }

    private createDiscordBotConstruct(): DiscordBotConstruct {
        return new DiscordBotConstruct(this, "DiscordBotConstruct", {
            commandsLambdaFunction: this.commandLambda,
        });
    }

    private createImageHostBucket(): Bucket {
        return new Bucket(this, "ImageHostBucket", {
            blockPublicAccess: new BlockPublicAccess({
                blockPublicPolicy: false,
                restrictPublicBuckets: false,
            }),
            enforceSSL: true,   
            publicReadAccess: true
        });
    }
}
