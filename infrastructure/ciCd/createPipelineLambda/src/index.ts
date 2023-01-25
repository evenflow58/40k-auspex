import {
  CloudFormationClient,
  CreateStackCommand,
  CreateStackCommandInput,
} from "@aws-sdk/client-cloudformation";
import {S3Client, PutObjectCommand, PutObjectCommandInput} from "@aws-sdk/client-s3"

export const handler = async (event: {
  BranchName: string;
  TemplateUrl: string;
}): Promise<any> => {
  console.log("event", event);

  const branchName = event.BranchName;
  if (branchName === "master") {
    console.log("Not creating anything because this is the master branch.");
    return;
  }

  console.log("Uploading template for tagging")
  const templateConfigurationFile = {
    Tags: {
      BranchName: branchName
    }
  }
  const fileName = `${branchName}-template-config-file`
  const s3Client = new S3Client({ region: "us-east-1" });
  const s3Input: PutObjectCommandInput = {
    Body: JSON.stringify(templateConfigurationFile),
    Bucket: "monster-of-week-code",
    Key: fileName,
  }
  const s3Command = new PutObjectCommand(s3Input);
  const response = await s3Client.send(s3Command);

  console.log("response", response);

  // Uploading and running pipeline
  const client = new CloudFormationClient({ region: "us-east-1" });
  const input: CreateStackCommandInput = {
    StackName: `monster-week-${branchName}`,
    TemplateURL: event.TemplateUrl,
    Parameters: [
      {
        ParameterKey: "GitHubBranch",
        ParameterValue: branchName,
        TemplateUrl: "",
      },
    ],
    OnFailure: "ROLLBACK",
    Capabilities: ["CAPABILITY_NAMED_IAM"],
  };
  const command = new CreateStackCommand(input);

  console.log(`Creating environment for ${branchName}`);

  return await client.send(command);
};
