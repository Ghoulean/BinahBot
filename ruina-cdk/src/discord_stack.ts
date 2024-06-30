import { Duration, Stack, StackProps } from "aws-cdk-lib";
import {
    Cors,
    LambdaIntegration,
    RequestValidator,
    RestApi,
} from "aws-cdk-lib/aws-apigateway";
import { AttributeType, TableV2 } from "aws-cdk-lib/aws-dynamodb";
import {
    Effect,
    Policy,
    PolicyDocument,
    PolicyStatement,
} from "aws-cdk-lib/aws-iam";
import * as lambda from "aws-cdk-lib/aws-lambda";
import { Architecture, Code, Runtime } from "aws-cdk-lib/aws-lambda";
import { BlockPublicAccess, Bucket } from "aws-cdk-lib/aws-s3";
import { Secret } from "aws-cdk-lib/aws-secretsmanager";
import { Construct } from "constructs";

// todo: move to (more) sane location with npm script
const BINAHBOT_BOOTSTRAP_LOCATION = `../rust/target/lambda/binah_bot/bootstrap.zip`;
const THUMBNAIL_BOOTSTRAP_LOCATION = `../rust/target/lambda/thumbnail/bootstrap.zip`;

// The handler value syntax is `{cargo-package-name}.{bin-name}`.
// source: https://github.com/codetalkio/patterns-serverless-rust-minimal/blob/bb36e3dc1a28d709511c4252f7bf880c363fccdb/deployment/lib/lambda-stack.ts#L23C25-L23C90
const BINAHBOT_FUNCTION_HANDLER = "binah_bot.bootstrap";
const THUMBNAIL_FUNCTION_HANDLER = "thumbnail.bootstrap";

export interface DiscordStackProps extends StackProps {
    clientId: string;
    emojis: {
        [key: string]: string | undefined;
    };
}

export class DiscordStack extends Stack {
    private readonly discordBotLambda: lambda.Function;
    private readonly thumbnailLambda: lambda.Function;
    private readonly discordAPISecrets: Secret;
    private readonly apigw: RestApi;
    private readonly imageHostBucket: Bucket;
    private readonly deckRepository: TableV2;

    constructor(scope: Construct, id: string, props: DiscordStackProps) {
        super(scope, id);
        this.discordBotLambda = this.createDiscordBotLambda();
        this.thumbnailLambda = this.createThumbnailGenerationLambda();
        this.discordAPISecrets = this.createSecret();
        this.apigw = this.createApigw();
        this.imageHostBucket = this.createImageHostBucket();
        this.deckRepository = this.createDeckRepository();

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
        this.discordBotLambda.addEnvironment(
            "DECK_REPOSITORY_NAME",
            this.deckRepository.tableName
        );
        this.discordBotLambda.addEnvironment("CLIENT_ID", props.clientId);
        this.discordBotLambda.addEnvironment(
            "THUMBNAIL_LAMBDA_ARN",
            this.thumbnailLambda.functionArn
        );

        this.thumbnailLambda.addEnvironment(
            "S3_BUCKET_NAME", 
            this.imageHostBucket.bucketName
        );
        this.thumbnailLambda.addEnvironment(
            "S3_DIRECTORY", 
            "deck_thumbnails"
        );

        this.deckRepository.grantReadWriteData(this.discordBotLambda);
        this.createBucketDeckThumbnailWriteAccessPolicy(
            this.imageHostBucket
        ).forEach((statement) => {
            this.thumbnailLambda.addToRolePolicy(statement);
        });
    }

    private createSecret(): Secret {
        return new Secret(this, "discord-bot-api-key");
    }

    private createDiscordBotLambda(): lambda.Function {
        return new lambda.Function(this, "DiscordBotFunction", {
            runtime: Runtime.PROVIDED_AL2023,
            handler: BINAHBOT_FUNCTION_HANDLER,
            code: Code.fromAsset(BINAHBOT_BOOTSTRAP_LOCATION),
            timeout: Duration.seconds(5),
            memorySize: 512,
            architecture: Architecture.ARM_64,
        });
    }

    private createThumbnailGenerationLambda(): lambda.Function {
        return new lambda.Function(this, "ThumbnailFunction", {
            runtime: Runtime.PROVIDED_AL2023,
            handler: THUMBNAIL_FUNCTION_HANDLER,
            code: Code.fromAsset(THUMBNAIL_BOOTSTRAP_LOCATION),
            timeout: Duration.seconds(30),
            memorySize: 512,
            architecture: Architecture.ARM_64,
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

    private createBucketDeckThumbnailWriteAccessPolicy(
        bucket: Bucket
    ): PolicyStatement[] {
        return [
            new PolicyStatement({
                effect: Effect.ALLOW,
                actions: ["s3:PutObject", "s3:GetObject"],
                resources: [`${bucket.bucketArn}/deck_thumbnails/*`],
            }),
            new PolicyStatement({
                effect: Effect.ALLOW,
                actions: ["s3:ListBucket"],
                resources: [bucket.bucketArn],
                conditions: {
                    StringLike: {
                        "s3:prefix": ["deck_thumbnails/*"],
                    },
                },
            }),
        ];
    }

    private createDeckRepository(): TableV2 {
        return new TableV2(this, "DeckRepositoryTable", {
            partitionKey: { name: "author", type: AttributeType.STRING },
            sortKey: { name: "deck_name", type: AttributeType.STRING },
            deletionProtection: true,
            globalSecondaryIndexes: [
                {
                    indexName: "gsi1",
                    partitionKey: {
                        name: "keypage",
                        type: AttributeType.STRING,
                    },
                    sortKey: { name: "author", type: AttributeType.STRING },
                },
            ],
            pointInTimeRecovery: true,
            tableName: "DeckRepository",
        });
    }
}
