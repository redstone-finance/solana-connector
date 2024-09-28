#!/bin/bash

set -e

# Load environment variables from .env file
if [ -f .env ]; then
  export $(cat .env | sed 's/#.*//g' | xargs)
else
  echo ".env file not found"
  exit 1
fi

# Check if required environment variables are set
if [ -z "$PRIVATE_KEY" ] || [ -z "$RPC_URL" ]; then
  echo "PRIVATE_KEY or RPC_URL is not set in .env file"
  exit 1
fi

# Set your Google Cloud project ID and region
PROJECT_ID="piotrostr-resources"
REGION="europe-central2"

./push-image.sh

# Deploy to Cloud Run
gcloud run deploy redstone-pusher \
  --image $IMAGE_NAME \
  --platform managed \
  --region $REGION \
  --set-env-vars "PRIVATE_KEY=$PRIVATE_KEY,RPC_URL=$RPC_URL" \
  --allow-unauthenticated

# Create or update Cloud Scheduler job
CLOUD_RUN_URL=$(gcloud run services describe redstone-pusher \
    --platform managed \
    --region $REGION \
    --format 'value(status.url)')

gcloud scheduler jobs update http redstone-pusher-job \
  --schedule="* * * * *" \
  --uri="$CLOUD_RUN_URL" \
  --http-method=GET \
  --attempt-deadline=540s \
  --time-zone="UTC" \
  || gcloud scheduler jobs create http redstone-pusher-job \
     --schedule="* * * * *" \
     --uri="$CLOUD_RUN_URL" \
     --http-method=GET \
     --attempt-deadline=540s \
     --time-zone="UTC"

