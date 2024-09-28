#!/bin/bash

# Set project_id as a constant
PROJECT_ID="piotrostr-resources"

# Function to check if a permission already exists
check_permission() {
    local project=$1
    local member=$2
    local role=$3
    gcloud projects get-iam-policy $project --format=json | \
        jq -e ".bindings[] | select(.role == \"$role\" and .members[] == \"$member\")" > /dev/null
}

# Add run.admin role if missing
if ! check_permission $PROJECT_ID serviceAccount:103617211893@cloudbuild.gserviceaccount.com roles/run.admin; then
    gcloud projects add-iam-policy-binding $PROJECT_ID \
     --member=serviceAccount:103617211893@cloudbuild.gserviceaccount.com \
     --role=roles/run.admin
    echo "Added roles/run.admin permission"
else
    echo "roles/run.admin permission already exists"
fi

# Add iam.serviceAccountUser role if missing
if ! check_permission $PROJECT_ID serviceAccount:103617211893@cloudbuild.gserviceaccount.com roles/iam.serviceAccountUser; then
    gcloud projects add-iam-policy-binding $PROJECT_ID \
     --member=serviceAccount:103617211893@cloudbuild.gserviceaccount.com \
     --role=roles/iam.serviceAccountUser
    echo "Added roles/iam.serviceAccountUser permission"
else
    echo "roles/iam.serviceAccountUser permission already exists"
fi

# check if local build and test pass
npm run build && bun test && gcloud builds submit \
    --config=cloudbuild.yaml \
    --substitutions=_PRIVATE_KEY="$(grep PRIVATE_KEY .env | cut -d '=' -f2)",_RPC_URL="$(grep RPC_URL .env | cut -d '=' -f2)" 
