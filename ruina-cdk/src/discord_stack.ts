import { Duration, Stack, StackProps } from "aws-cdk-lib";
import {
    Cors,
    LambdaIntegration,
    RequestValidator,
    RestApi,
} from "aws-cdk-lib/aws-apigateway";
import { Code, Function, Runtime } from "aws-cdk-lib/aws-lambda";
import { BlockPublicAccess, Bucket } from "aws-cdk-lib/aws-s3";
import { Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

export class DiscordStack extends Stack {
    private readonly discordBotLambda: Function;
    private readonly discordAPISecrets: Secret;
    private readonly apigw: RestApi;
    private readonly imageHostBucket: Bucket;

    constructor(scope: Construct, id: string, props: StackProps) {
        super(scope, id);
        this.discordBotLambda = this.createDiscordBotLambda();
        this.discordAPISecrets = this.createSecret();
        this.apigw = this.createApigw();
        this.imageHostBucket = this.createImageHostBucket();

        this.discordAPISecrets.grantRead(this.discordBotLambda);
        this.discordBotLambda.addEnvironment(
            "S3_BUCKET_NAME",
            this.imageHostBucket.bucketName
        );
        this.discordBotLambda.addEnvironment(
            "SECRETS_ID",
            this.discordAPISecrets.secretName
        );
    }

    private createSecret(): Secret {
        return new Secret(this, "discord-bot-api-key");
    }

    private createDiscordBotLambda(): Function {
        return new Function(this, "DiscordBotFunction", {
            runtime: Runtime.NODEJS_18_X,
            handler: "build/src.handler",
            code: Code.fromAsset("../ruina-discord-bot/ruina-discord-bot.zip"),
            timeout: Duration.seconds(30),
            memorySize: 512,
        });
    }

    private createApigw(): RestApi {
        // Create our API Gateway
        const discordBotAPI = new RestApi(this, "discord-bot-api", {
            defaultCorsPreflightOptions: {
                allowOrigins: Cors.ALL_ORIGINS,
            },
        });
        const discordBotAPIValidator = new RequestValidator(
            this,
            "discord-bot-api-validator",
            {
                restApi: discordBotAPI,
                validateRequestBody: true,
                validateRequestParameters: true,
            }
        );

        // User authentication endpoint configuration
        const discordBotEventItems = discordBotAPI.root.addResource("event", {
            defaultCorsPreflightOptions: {
                allowOrigins: ["*"],
            },
        });

        // Transform our requests and responses as appropriate.
        const discordBotIntegration: LambdaIntegration = new LambdaIntegration(
            this.discordBotLambda,
            {
                proxy: true,
                integrationResponses: [
                    {
                        statusCode: "200",
                    },
                    {
                        statusCode: "401",
                    },
                ],
            }
        );

        // Add a POST method for the Discord APIs.
        discordBotEventItems.addMethod("POST", discordBotIntegration, {
            apiKeyRequired: false,
            requestValidator: discordBotAPIValidator,
            methodResponses: [
                {
                    statusCode: "200",
                },
                {
                    statusCode: "401",
                },
            ],
        });

        return discordBotAPI;
    }

    private createImageHostBucket(): Bucket {
        return new Bucket(this, "ImageHostBucket", {
            blockPublicAccess: new BlockPublicAccess({
                blockPublicPolicy: false,
                restrictPublicBuckets: false,
            }),
            enforceSSL: true,
            publicReadAccess: true,
        });
    }
}
