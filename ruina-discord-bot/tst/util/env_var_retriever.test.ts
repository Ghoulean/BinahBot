import { EnvVarRetriever } from "../../src/util/env_var_retriever";

const ENV_VAR_KEY = "testEnvVarKey";
const ENV_VAR_VALUE = "testEnvVarValue";
const ENV_VAR_KEY_DOES_NOT_EXIST = "testEnvVarKeyDNE";

let envVarRetriever: EnvVarRetriever;

beforeAll(() => {
    process.env[ENV_VAR_KEY] = ENV_VAR_VALUE;
    delete process.env[ENV_VAR_KEY_DOES_NOT_EXIST];
});

afterAll(() => {
    delete process.env[ENV_VAR_KEY];
});

beforeEach(() => {
    envVarRetriever = new EnvVarRetriever();
});

test("should contain env var if key is set", () => {
    expect(envVarRetriever.contains(ENV_VAR_KEY)).toBe(true);
});

test("should not contain env var if key is not set", () => {
    expect(envVarRetriever.contains(ENV_VAR_KEY_DOES_NOT_EXIST)).toBe(false);
});

test("should get env var with key", () => {
    expect(envVarRetriever.get(ENV_VAR_KEY)).toBe(ENV_VAR_VALUE);
});

test("should get undefined if key is not set", () => {
    expect(envVarRetriever.get(ENV_VAR_KEY_DOES_NOT_EXIST)).toBe(undefined);
});

test("should get required env var with key", () => {
    expect(envVarRetriever.getRequired(ENV_VAR_KEY)).toBe(ENV_VAR_VALUE);
});

test("should throw error when get required but key is not set", () => {
    expect(() => {
        envVarRetriever.getRequired(ENV_VAR_KEY_DOES_NOT_EXIST);
    }).toThrow();
});
