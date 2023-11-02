import { S3Client, ListObjectsCommand, ListObjectsCommandInput, DeleteObjectsCommandInput, ObjectIdentifier, DeleteObjectsCommand, ListBucketsCommand } from "@aws-sdk/client-s3";

export const handler = async (event: { BranchName: string }): Promise<any> => {
  const stackPrefix = `${event.BranchName.split('-').map(word => word.charAt(0)).join('')}-`;

  const buckets = await getBuckets(stackPrefix);

  const emptyRequests = await Promise.allSettled(buckets.map(emptyBucket))

  console.log('emptyRequests', emptyRequests);

  if (emptyRequests.filter(({ status }) => status === 'rejected')?.length > 0) {
    return { result: 'BUCKET_FAILURE'};
  }

  return { prefix: stackPrefix };
}

const getBuckets = async (prefix: string): Promise<Array<string>> => {
  const s3Client = new S3Client({ region: 'us-east-1' });

  const buckets = await s3Client.send(new ListBucketsCommand({}));

  return buckets?.Buckets
    .filter(({ Name }) => Name.startsWith(prefix))
    .map(({ Name }) => Name);
}

const emptyBucket = async (bucketName: string): Promise<void> => {
  const s3Client = new S3Client({ region: 'us-east-1' });

  const listObjectsCommandInput: ListObjectsCommandInput = {
    Bucket: bucketName
  }

  const objects = await s3Client.send(new ListObjectsCommand(listObjectsCommandInput));

  if (!objects.Contents) return;

  const deleteObjectsCommandInput: DeleteObjectsCommandInput = {
    Bucket: bucketName,
    Delete: {
      Objects: objects.Contents.map(object => ({ Key: object.Key} as ObjectIdentifier))
    }
  }
  await s3Client.send(new DeleteObjectsCommand(deleteObjectsCommandInput));
}