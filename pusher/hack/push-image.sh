#!/bin/bash

PROJECT_ID=piotrostr-resources

gcloud auth configure-docker

docker build -t gcr.io/$PROJECT_ID/redstone-pusher --platform linux/amd64 .
docker push gcr.io/$PROJECT_ID/redstone-pusher
