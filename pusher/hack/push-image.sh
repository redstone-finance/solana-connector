#!/bin/bash

PROJECT_ID=piotrostr-resources

gcloud auth configure-docker

docker build -t gcr.io/$PROJECT_ID/redstone-pusher .
docker push gcr.io/$PROJECT_ID/redstone-pusher
