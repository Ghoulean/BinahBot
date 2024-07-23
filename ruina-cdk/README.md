# ruina-cdk

Deploys BinahBot to AWS

## Deployment instructions:
Populate `.env` with `CLIENT_ID="<Discord application id>"`. You may optionally fill out emoji ids. See the `.env.example`.

1. Follow the instructions to set up the Discord bot application and to build the Rust lambdas. You need to rebuild the lambdas every time you want to deploy a new change.
2. Set up an AWS profile
3. `npm run build`
4. `npm run cdk deploy -- --profile <your AWS profile name here>`

The following instructions are for first-time setup after running the initial deployment:
1. Go to AWS Secrets Manager and insert the following secret values, filling out each field accordingly:

```
{
    "authToken":"<DISCORD_AUTH_TOKEN_HERE>",
    "publicKey":"<DISCORD_PUBLIC_KEY_HERE>",
    "applicationId":"<DISCORD_APPLICATION_ID_HERE>",
    "botToken":"<DISCORD_BOT_TOKEN_HERE>"
}
```

2. Go to your s3 bucket and create the `deck_thumbnails` subdirectory (for user-submitted deck thumbnails)
3. Upload game assets
    1. Upload all abno and combat page images into the root directory
    2. Upload key page images into `/Sprite` directory
    3. Upload battle symbol images into `/battle_symbol` directory
4. Copy your AWS API Gateway endpoint URL for `POST /event` into your Discord application Interaction endpoint URL field. Your URL should look like `https://<APIGW ID>.execute-api.<AWS region>.amazonaws.com/prod/event`

## Why AWS?
1. great question
