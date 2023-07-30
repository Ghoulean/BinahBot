/**
 * Object holding Discord secrets stored in AWS Secrets Manager
 * 
 */
export type DiscordSecrets = {
    applicationId: string;
    authToken: string;
    publicKey: string;
}
