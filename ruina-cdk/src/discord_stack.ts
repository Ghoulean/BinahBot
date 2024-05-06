import { Duration, Stack, StackProps } from "aws-cdk-lib";
import {
    Cors,
    LambdaIntegration,
    RequestValidator,
    RestApi,
} from "aws-cdk-lib/aws-apigateway";
import { Architecture, Code, Function, Runtime } from "aws-cdk-lib/aws-lambda";
import { BlockPublicAccess, Bucket } from "aws-cdk-lib/aws-s3";
import { Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

// todo: move to (more) sane location with npm script
const BOOTSTRAP_LOCATION = `../rust/target/lambda/binah_bot/bootstrap.zip`;

// The handler value syntax is `{cargo-package-name}.{bin-name}`.
// source: https://github.com/codetalkio/patterns-serverless-rust-minimal/blob/bb36e3dc1a28d709511c4252f7bf880c363fccdb/deployment/lib/lambda-stack.ts#L23C25-L23C90
const FUNCTION_HANDLER = "binah_bot.bootstrap";

export interface DiscordStackProps extends StackProps {
    clientId: string;
    emojis: {
        [key: string]: string | undefined
    }
}

export class DiscordStack extends Stack {
    private readonly discordBotLambda: Function;
    private readonly discordAPISecrets: Secret;
    private readonly apigw: RestApi;
    private readonly imageHostBucket: Bucket;

    constructor(scope: Construct, id: string, props: DiscordStackProps) {
        super(scope, id);
        this.discordBotLambda = this.createDiscordBotLambda();
        this.discordAPISecrets = this.createSecret();
        this.apigw = this.createApigw();
        this.imageHostBucket = this.createImageHostBucket();

        this.discordAPISecrets.grantRead(this.discordBotLambda);

        for (const [key, val] of Object.entries(props.emojis)) {
            if (val) {
                this.discordBotLambda.addEnvironment(key, val);
            }
        }
        this.discordBotLambda.addEnvironment(
            "S3_BUCKET_NAME",
            this.imageHostBucket.bucketName
        );
        this.discordBotLambda.addEnvironment(
            "SECRETS_ID",
            this.discordAPISecrets.secretName
        );
        this.discordBotLambda.addEnvironment("CLIENT_ID", props.clientId);

    }

    private createSecret(): Secret {
        return new Secret(this, "discord-bot-api-key");
    }

    private createDiscordBotLambda(): Function {
        return new Function(this, "DiscordBotFunction", {
            runtime: Runtime.PROVIDED_AL2023,
            handler: FUNCTION_HANDLER,
            code: Code.fromAsset(BOOTSTRAP_LOCATION),
            timeout: Duration.seconds(30),
            memorySize: 512,
            architecture: Architecture.ARM_64
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
