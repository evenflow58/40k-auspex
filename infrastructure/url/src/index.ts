// import {
//   S3Client,
//   GetObjectCommand,
//   GetObjectCommandInput,
// } from "@aws-sdk/client-s3";

export const handler = async (event: any): Promise<any> => {
  console.log(event);

  // const input: GetObjectCommandInput = {
  //   Bucket: "monster-of-the-week-ui",
  //   Key: "serve-site-from-edge-lambda/",
  // };
  // const command = new GetObjectCommand(input);
  // const client = new S3Client({ region: "us-east-1" });
  // const files = await client.send(command);

  // console.log("got files");

  const response = {
    status: 200,
    statusDescription: "OK",
    body: "Lambda@Edge is awesome! Part 3!",
  };

  return response;
};
