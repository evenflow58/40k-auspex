import { PutObjectCommand, PutObjectCommandInput, S3Client } from "@aws-sdk/client-s3";

export const createFile = async (branchName: string): Promise<void> => {
    const client = new S3Client({ region: 'us-east-1' });

    const templateConfigurationFile = {
        Tags: {
            Env: branchName
        }
    };

    const input: PutObjectCommandInput = {
        Bucket: "",
        Key: `${branchName}-templateConfigurationFile`,
        Body: JSON.stringify(templateConfigurationFile),
    };

    await client.send(new PutObjectCommand(input));
};