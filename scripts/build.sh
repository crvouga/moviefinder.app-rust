#!/bin/bash

echo "Running db migrations"

npx dbmate -e DATABASE_URL up

echo "Running db migrations done"

