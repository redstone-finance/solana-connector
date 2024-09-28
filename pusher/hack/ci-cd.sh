#!/bin/bash

gcloud projects add-iam-policy-binding piotrostr-resources \
 --member=serviceAccount:103617211893@cloudbuild.gserviceaccount.com \
 --role=roles/run.admin

gcloud projects add-iam-policy-binding piotrostr-resources \
 --member=serviceAccount:103617211893@cloudbuild.gserviceaccount.com \
 --role=roles/iam.serviceAccountUser

gcloud builds submit \
    --config=cloudbuild.yaml \
    --substitutions=_PRIVATE_KEY="$(grep PRIVATE_KEY .env | cut -d '=' -f2)",_RPC_URL="$(grep RPC_URL .env | cut -d '=' -f2)" \
    --enable-cache

