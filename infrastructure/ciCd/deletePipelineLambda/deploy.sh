#!/bin/bash

# Build the application
npm run build;

# Get the location of the newly uploaded definition file
# jq install required
version=$( \
    aws s3api put-object \
    --bucket auspex-code-repo-bucket \
    --key delete-stacks-definition.json \
    --body state-definition.json \
    | jq -r '.VersionId');

sam build;

sam deploy \
    --capabilities CAPABILITY_NAMED_IAM CAPABILITY_AUTO_EXPAND \
    --no-confirm-changeset \
    --parameter-overrides DefinitionS3VersionId=$version;